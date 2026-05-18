import os
import re
import sys
import yaml
import requests
from pathlib import Path
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.metrics.pairwise import cosine_similarity
from collections import namedtuple

Post = namedtuple('Post', ['metadata', 'content'])

ROOT = Path(".")
SKILLS_DIR = ROOT / "skills"

GITHUB_TOKEN = os.getenv("GITHUB_TOKEN")
PR_NUMBER = os.getenv("PR_NUMBER")
REPO_NAME = os.getenv("REPO_NAME")

VALID_CATEGORIES = {
    "frontend",
    "backend",
    "architecture",
    "dev-tools",
    "ui-ux",
    "code-quality"
}

VALID_SKILL_TYPES = {
    "workflow",
    "architecture",
    "security",
    "debugging",
    "review",
    "generation",
    "migration"
}

VALID_SECURITY_LEVELS = {
    "safe",
    "review-required",
    "dangerous"
}

VALID_AGENTS = {
    "claude-code",
    "cursor",
    "copilot",
    "codex",
    "gemini"
}

REQUIRED_FIELDS = [
    "name",
    "description",
    "version",
    "category",
    "tags",
    "skill_type",
    "author",
    "license",
    "compatible_agents",
    "estimated_context_tokens",
    "dangerous",
    "requires_review",
    "security_level",
    "dependencies",
    "triggers",
    "permissions",
    "input_requirements",
    "output_contract",
    "failure_conditions",
    "last_updated"
]

REQUIRED_SECTIONS = [
    "## Purpose",
    "## When to use",
    "## When NOT to use",
    "## Inputs required",
    "## Workflow",
    "## Rules",
    "## Anti-patterns",
    "## Failure conditions",
    "## Validation checklist",
    "## Output format",
    "## Security considerations",
    "## Agent execution notes",
    "## Example"
]

API_KEY_PATTERNS = [
    r"ghp_[A-Za-z0-9]{36}",
    r"AIza[0-9A-Za-z\-_]{35}",  # Fixed: changed \\ to \
    r"sk-[A-Za-z0-9]{20,}",
    r"AKIA[0-9A-Z]{16}",
    r"-----BEGIN (?:RSA|DSA|EC|OPENSSH) PRIVATE KEY-----",
    r"xox[baprs]-[A-Za-z0-9-]+",
]

SUSPICIOUS_PATTERNS = [
    r"curl.+\|.+bash",
    r"wget.+\|.+sh",
    r"powershell.+iex",
    r"Invoke-Expression",
    r"base64\s+-d",
    r"eval\s*[(]",
    r"exec\s*[(]",
    r"os\.system\s*[(]",
    r"subprocess\.",
    r"chmod\s+777",
]

OBFUSCATION_PATTERNS = [
    r"[A-Za-z0-9+/]{200,}={0,2}",
    r"fromcharcode",
    r"atob\s*[(]",
    r"btoa\s*[(]",
    r"marshal\.loads",
    r"zlib\.decompress",
    r"exec\s*[(]base64",
]

REQUIRED_PR_CHECKBOXES = [
    "I have read and accepted the DISCLAIMER and CONTRIBUTING GUIDELINES",
    "I am making chages that are actually useful and that they do not violate the SECURITY GUIDELINES"
]

errors = []
warnings = []


def fail(message):
    errors.append(message)


def warn(message):
    warnings.append(message)



def validate_pr_template():
    if not all([GITHUB_TOKEN, PR_NUMBER, REPO_NAME]):
        fail("Missing GitHub environment variables")
        return

    url = f"https://api.github.com/repos/{REPO_NAME}/pulls/{PR_NUMBER}"

    headers = {
        "Authorization": f"Bearer {GITHUB_TOKEN}",
        "Accept": "application/vnd.github+json"
    }

    response = requests.get(url, headers=headers)

    if response.status_code != 200:
        fail(f"Failed to fetch PR data: {response.status_code}")
        return

    data = response.json()
    body = data.get("body", "")

    if len(body.strip()) < 200:
        fail("PR description is too short")

    for checkbox in REQUIRED_PR_CHECKBOXES:
        pattern = rf"- \[x\] {re.escape(checkbox)}"

        if not re.search(pattern, body, re.IGNORECASE):
            fail(f"Required PR checkbox missing: {checkbox}")

    required_sections = [
        "## Summary",
        "## Type of Change",
        "## What Changed",
        "## Validation"
    ]

    for section in required_sections:
        if section not in body:
            fail(f"Missing PR section: {section}")



def validate_frontmatter(metadata, path):
    for field in REQUIRED_FIELDS:
        if field not in metadata:
            fail(f"{path}: Missing metadata field '{field}'")

    name = metadata.get("name", "")

    if not re.fullmatch(r"[a-z0-9-]{1,40}", name):
        fail(f"{path}: Invalid kebab-case name")

    version = metadata.get("version", "")

    if not re.fullmatch(r"\d+\.\d+\.\d+", str(version)):
        fail(f"{path}: Invalid semver version")

    description = metadata.get("description", "")

    if len(description) > 100:
        fail(f"{path}: Description exceeds 100 characters")

    if not description.endswith("."):
        fail(f"{path}: Description must end with a period")

    category = metadata.get("category")

    if category not in VALID_CATEGORIES:
        fail(f"{path}: Invalid category")

    skill_type = metadata.get("skill_type")

    if skill_type not in VALID_SKILL_TYPES:
        fail(f"{path}: Invalid skill_type")

    security_level = metadata.get("security_level")

    if security_level not in VALID_SECURITY_LEVELS:
        fail(f"{path}: Invalid security_level")

    compatible_agents = metadata.get("compatible_agents", [])

    for agent in compatible_agents:
        if agent not in VALID_AGENTS:
            fail(f"{path}: Invalid compatible agent '{agent}'")

    permissions = metadata.get("permissions", {})

    try:
        shell_execute = permissions["shell"]["execute"]
        dangerous = metadata.get("dangerous")

        if shell_execute and dangerous is False:
            fail(f"{path}: shell.execute=true requires dangerous=true")
    except Exception:
        fail(f"{path}: Invalid permissions structure")



def validate_sections(content, path):
    last_index = -1

    for section in REQUIRED_SECTIONS:
        index = content.find(section)

        if index == -1:
            fail(f"{path}: Missing required section '{section}'")
            continue

        if index < last_index:
            fail(f"{path}: Section order invalid near '{section}'")

        last_index = index



def scan_api_keys(content, path):
    for pattern in API_KEY_PATTERNS:
        if re.search(pattern, content, re.IGNORECASE):
            fail(f"{path}: Potential API key or secret detected")



def scan_suspicious_commands(content, path):
    for pattern in SUSPICIOUS_PATTERNS:
        try:
            if re.search(pattern, content, re.IGNORECASE):
                fail(f"{path}: Suspicious command detected")
        except Exception as e:
            print(f"CRITICAL DEBUG: Failed evaluating pattern '{pattern}' on file '{path}'")
            raise e


def scan_obfuscation(content, path):
    for pattern in OBFUSCATION_PATTERNS:
        if re.search(pattern, content, re.IGNORECASE):
            fail(f"{path}: Possible obfuscated payload detected")



def detect_duplicates(skill_texts):
    if len(skill_texts) < 2:
        return

    names = list(skill_texts.keys())
    texts = list(skill_texts.values())

    vectorizer = TfidfVectorizer(stop_words="english")
    matrix = vectorizer.fit_transform(texts)

    similarity = cosine_similarity(matrix)

    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            score = similarity[i][j]

            if score > 0.90:
                fail(
                    f"Duplicate skill detected between '{names[i]}' and '{names[j]}' (similarity {score:.2f})"
                )



def parse_frontmatter(content):
    """Parse YAML frontmatter from markdown content."""
    if not content.startswith('---'):
        raise ValueError("Content does not start with ---")
    
    parts = content.split('---', 2)
    if len(parts) < 3:
        raise ValueError("Invalid frontmatter format")
    
    metadata_str = parts[1]
    content_str = parts[2]
    
    try:
        metadata = yaml.safe_load(metadata_str)
    except yaml.YAMLError as e:
        raise ValueError(f"Invalid YAML: {e}")
    
    if metadata is None:
        metadata = {}
    
    return Post(metadata=metadata, content=content_str)


def validate_skill_file(path):
    raw = path.read_text(encoding="utf-8")

    try:
        post = parse_frontmatter(raw)
    except Exception as e:
        fail(f"{path}: Invalid frontmatter: {e}")
        return None

    validate_frontmatter(post.metadata, path)
    validate_sections(post.content, path)
    scan_api_keys(raw, path)
    scan_suspicious_commands(raw, path)
    scan_obfuscation(raw, path)

    if "**❌ Anti-pattern:**" not in raw:
        fail(f"{path}: Missing anti-pattern example")

    if "**✅ Correct pattern:**" not in raw:
        fail(f"{path}: Missing correct-pattern example")

    return post.content



def main():
    validate_pr_template()

    skill_texts = {}

    for skill_file in SKILLS_DIR.rglob("SKILL.md"):
        content = validate_skill_file(skill_file)

        if content:
            skill_texts[str(skill_file)] = content

    detect_duplicates(skill_texts)

    print("\n================ VALIDATION REPORT ================\n")

    if warnings:
        for warning in warnings:
            print(f"WARNING: {warning}")

    if errors:
        for error in errors:
            print(f"ERROR: {error}")

        print("\nValidation failed.")
        sys.exit(1)

    print("All automated checks passed.")
    print("Human review is still REQUIRED before merge.")

    sys.exit(0)


if __name__ == "__main__":
    main()
