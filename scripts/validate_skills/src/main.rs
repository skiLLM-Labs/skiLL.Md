use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

const VALID_CATEGORIES: &[&str] = &[
    "frontend", "backend", "architecture", "dev-tools", "ui-ux", "code-quality",
];
const VALID_SKILL_TYPES: &[&str] = &[
    "workflow", "architecture", "security", "debugging", "review", "generation", "migration",
];
const VALID_SECURITY_LEVELS: &[&str] = &["safe", "review-required", "dangerous"];
const VALID_AGENTS: &[&str] = &["claude-code", "cursor", "copilot", "codex", "gemini"];
const REQUIRED_FIELDS: &[&str] = &[
    "name", "description", "version", "category", "tags", "skill_type", "author", "license",
    "compatible_agents", "estimated_context_tokens", "dangerous", "requires_review",
    "security_level", "dependencies", "triggers", "permissions", "input_requirements",
    "output_contract", "failure_conditions", "last_updated",
];
const REQUIRED_SECTIONS: &[&str] = &[
    "## Purpose", "## When to use", "## When NOT to use", "## Inputs required", "## Workflow",
    "## Rules", "## Anti-patterns", "## Failure conditions", "## Validation checklist",
    "## Output format", "## Security considerations", "## Agent execution notes", "## Example",
];
const REQUIRED_PR_CHECKBOXES: &[&str] = &[
    "I have read and accepted the DISCLAIMER and CONTRIBUTING GUIDELINES",
    "I am making chages that are actually useful and that they do not violate the SECURITY GUIDELINES",
];

lazy_static! {
    static ref API_KEY_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)ghp_[A-Za-z0-9]{36}").unwrap(),
        Regex::new(r"(?i)AIza[0-9A-Za-z\-_]{35}").unwrap(),
        Regex::new(r"(?i)sk-[A-Za-z0-9]{20,}").unwrap(),
        Regex::new(r"(?i)AKIA[0-9A-Z]{16}").unwrap(),
        Regex::new(r"(?i)-----BEGIN (?:RSA|DSA|EC|OPENSSH) PRIVATE KEY-----").unwrap(),
        Regex::new(r"(?i)xox[baprs]-[A-Za-z0-9-]+").unwrap(),
    ];
    static ref SUSPICIOUS_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)curl.+\|.+bash").unwrap(),
        Regex::new(r"(?i)wget.+\|.+sh").unwrap(),
        Regex::new(r"(?i)curl\s+.*http.*\.sh").unwrap(),
        Regex::new(r"(?i)wget\s+.*http.*\.sh").unwrap(),
        Regex::new(r"(?i)curl\s+.*http.*\.exe").unwrap(),
        Regex::new(r"(?i)powershell.+iex").unwrap(),
        Regex::new(r"(?i)Invoke-Expression").unwrap(),
        Regex::new(r"(?i)base64\s+-d").unwrap(),
        Regex::new(r"(?i)sh\s+-c\s+.*http").unwrap(),
        Regex::new(r"(?i)rm\s+-rf\s+/").unwrap(),
    ];
    static ref SUSPICIOUS_INSTRUCTIONS: Vec<Regex> = vec![
        Regex::new(r"(?i)ignore (all )?previous instructions").unwrap(),
        Regex::new(r"(?i)you are now (an? )?(unrestricted|jailbroken|unfiltered)").unwrap(),
        Regex::new(r"(?i)disregard (the )?system prompt").unwrap(),
        Regex::new(r"(?i)bypass security").unwrap(),
        Regex::new(r"(?i)forget (all )?rules").unwrap(),
    ];
    static ref OBFUSCATION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)[A-Za-z0-9+/]{200,}={0,2}").unwrap(),
        Regex::new(r"(?i)marshal\.loads").unwrap(),
        Regex::new(r"(?i)zlib\.decompress").unwrap(),
    ];
    static ref HTML_COMMENT_REGEX: Regex = Regex::new(r"(?s)").unwrap();
}

struct Post {
    metadata: YamlValue,
    content: String,
}

struct Validator {
    errors: Vec<String>,
    warnings: Vec<String>,
}

impl Validator {
    fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    fn fail(&mut self, message: String) {
        self.errors.push(message);
    }

    #[allow(dead_code)]
    fn warn(&mut self, message: String) {
        self.warnings.push(message);
    }
}

fn get_pr_changed_files(client: &Client, token: &str, repo: &str, pr: &str) -> Vec<String> {
    if token.is_empty() || repo.is_empty() || pr.is_empty() {
        return Vec::new();
    }

    let url = format!("https://api.github.com/repos/{}/pulls/{}/files", repo, pr);
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/vnd.github+json")
        .header(USER_AGENT, "Rust-Skill-Validator")
        .send();

    match res {
        Ok(response) if response.status().is_success() => {
            if let Ok(json) = response.json::<Vec<JsonValue>>() {
                let files: Vec<String> = json
                    .iter()
                    .filter_map(|v| v["filename"].as_str().map(String::from))
                    .collect();
                println!("INFO: Found {} changed files from GitHub API.", files.len());
                for f in &files {
                    println!("  - {}", f);
                }
                return files;
            }
        }
        _ => println!("WARNING: Failed to fetch PR files from GitHub API."),
    }
    Vec::new()
}

fn extract_section(body: &str, section_name: &str) -> Option<String> {
    let start_idx = body.find(section_name)?;
    let content_start = start_idx + section_name.len();
    let rest = &body[content_start..];
    let end_idx = rest.find("\n## ").unwrap_or(rest.len());
    Some(rest[..end_idx].trim().to_string())
}

fn validate_pr_template(validator: &mut Validator, client: &Client, token: &str, repo: &str, pr: &str) {
    if token.is_empty() || repo.is_empty() || pr.is_empty() {
        println!("INFO: Skipping PR template validation (Running locally / Missing GitHub Env Vars).");
        return;
    }

    let url = format!("https://api.github.com/repos/{}/pulls/{}", repo, pr);
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .header(ACCEPT, "application/vnd.github+json")
        .header(USER_AGENT, "Rust-Skill-Validator")
        .send();

    let body = match res {
        Ok(response) if response.status().is_success() => {
            if let Ok(json) = response.json::<JsonValue>() {
                json["body"].as_str().unwrap_or("").to_string()
            } else {
                String::new()
            }
        }
        _ => {
            validator.fail("Failed to fetch PR data from GitHub API".into());
            return;
        }
    };

    let clean_body = HTML_COMMENT_REGEX.replace_all(&body, "").trim().to_string();

    let body_without_headings = clean_body.replace("## Summary", "")
                                          .replace("## Type of Change", "")
                                          .replace("## What Changed", "")
                                          .replace("## Files Added/Modified", "")
                                          .replace("## Skill Structure Checklist", "")
                                          .replace("## Validation", "")
                                          .replace("## Related Issues", "")
                                          .replace("## Screenshots / Preview", "")
                                          .replace("## Additional Notes", "");

    if body_without_headings.trim().len() < 20 {
        validator.fail("PR description content is essentially empty. Please replace the default template comments with actual details.".into());
        return;
    }

    for checkbox in REQUIRED_PR_CHECKBOXES {
        let cleaned = regex::escape(checkbox).replace("\\ ", "\\s+");
        let pattern = Regex::new(&format!(r"(?i)- \[[xX]\]\s+{}", cleaned)).unwrap();
        if !pattern.is_match(&body) {
            validator.fail(format!("Required PR checkbox missing or unchecked: {}", checkbox));
        }
    }

    let pr_sections = ["## Summary", "## Type of Change", "## What Changed"];

    for section_name in pr_sections {
        if !body.contains(section_name) {
            validator.fail(format!("Missing required PR section: {}", section_name));
            continue;
        }

        if let Some(content) = extract_section(&body, section_name) {
            let section_content = HTML_COMMENT_REGEX.replace_all(&content, "").trim().to_string();
            if section_content.is_empty() || section_content.starts_with("- [ ]") || section_content.len() < 5 {
                validator.fail(format!("The PR section '{}' has been left blank. Please provide actual details.", section_name));
            }
        }
    }
}

fn as_string(val: Option<&YamlValue>) -> String {
    match val {
        Some(YamlValue::String(s)) => s.clone(),
        Some(YamlValue::Number(n)) => n.to_string(),
        Some(YamlValue::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

fn validate_frontmatter(validator: &mut Validator, metadata: &YamlValue, path: &str) {
    let mapping = match metadata.as_mapping() {
        Some(m) => m,
        None => {
            validator.fail(format!("{}: Frontmatter is not a valid YAML mapping", path));
            return;
        }
    };

    for field in REQUIRED_FIELDS {
        if !mapping.contains_key(&YamlValue::String(field.to_string())) {
            validator.fail(format!("{}: Missing REQUIRED metadata field '{}'", path, field));
        }
    }

    let name = as_string(metadata.get("name"));
    if !Regex::new(r"^[a-z0-9-]{1,40}$").unwrap().is_match(&name) {
        validator.fail(format!("{}: Invalid 'name'. Must be lowercase kebab-case and under 40 chars.", path));
    }

    let version = as_string(metadata.get("version"));
    if !Regex::new(r"^\d+\.\d+\.\d+$").unwrap().is_match(&version) {
        validator.fail(format!("{}: Invalid 'version'. Must follow semver (e.g. 1.0.0).", path));
    }

    let category = as_string(metadata.get("category"));
    if !category.is_empty() && !VALID_CATEGORIES.contains(&category.as_str()) {
        validator.fail(format!("{}: Invalid 'category' '{}'", path, category));
    }

    let skill_type = as_string(metadata.get("skill_type"));
    if !skill_type.is_empty() && !VALID_SKILL_TYPES.contains(&skill_type.as_str()) {
        validator.fail(format!("{}: Invalid 'skill_type' '{}'", path, skill_type));
    }

    let security_level = as_string(metadata.get("security_level"));
    if !security_level.is_empty() && !VALID_SECURITY_LEVELS.contains(&security_level.as_str()) {
        validator.fail(format!("{}: Invalid 'security_level' '{}'", path, security_level));
    }
    
    let is_dangerous = as_string(metadata.get("dangerous")) == "true";
    if is_dangerous && security_level == "safe" {
        validator.fail(format!("{}: Conflict - 'dangerous' is true, but 'security_level' is safe.", path));
    }

    if let Some(YamlValue::Sequence(seq)) = metadata.get("compatible_agents") {
        for agent_val in seq {
            let agent = as_string(Some(agent_val));
            if !VALID_AGENTS.contains(&agent.as_str()) {
                validator.fail(format!("{}: Invalid compatible agent '{}' in frontmatter", path, agent));
            }
        }
    }
}

fn validate_sections(validator: &mut Validator, content: &str, path: &str) {
    let mut last_index = 0;
    for section in REQUIRED_SECTIONS {
        if let Some(index) = content.find(section) {
            if index < last_index {
                validator.fail(format!("{}: Section order invalid near '{}'. Please follow the schema order.", path, section));
            }
            last_index = index;
        } else {
            validator.fail(format!("{}: Missing required markdown section '{}'", path, section));
        }
    }
}

fn scan_patterns(validator: &mut Validator, content: &str, path: &str, patterns: &[Regex], message: &str) {
    for pattern in patterns {
        if pattern.is_match(content) {
            validator.fail(format!("{}: {} (Matched: {})", path, message, pattern.as_str()));
        }
    }
}

fn parse_frontmatter(content: &str) -> Result<Post, String> {
    if !content.starts_with("---") {
        return Err("Content does not start with --- (Missing YAML Frontmatter)".to_string());
    }
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err("Invalid frontmatter format. Ensure it is closed with ---".to_string());
    }
    
    let metadata_str = parts[1];
    let content_str = parts[2].to_string();
    
    let metadata: YamlValue = serde_yaml::from_str(metadata_str).unwrap_or(YamlValue::Mapping(Default::default()));
    Ok(Post { metadata, content: content_str })
}

fn detect_duplicates(validator: &mut Validator, skill_texts: &HashMap<String, String>) {
    if skill_texts.len() < 2 { return; }

    let names: Vec<_> = skill_texts.keys().collect();
    let texts: Vec<_> = skill_texts.values().collect();

    let mut df: HashMap<String, usize> = HashMap::new();
    let mut doc_tokens: Vec<Vec<String>> = Vec::new();

    for text in &texts {
        let tokens: Vec<String> = text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| s.len() > 2)
            .map(|s| s.to_string())
            .collect();

        let unique_tokens: HashSet<_> = tokens.iter().cloned().collect();
        for token in unique_tokens {
            *df.entry(token).or_insert(0) += 1;
        }
        doc_tokens.push(tokens);
    }

    let num_docs = texts.len() as f64;
    let mut idf: HashMap<String, f64> = HashMap::new();
    for (token, count) in df {
        idf.insert(token, ((1.0 + num_docs) / (1.0 + count as f64)).ln() + 1.0);
    }

    let mut vectors: Vec<HashMap<String, f64>> = Vec::new();
    for tokens in &doc_tokens {
        let mut tf: HashMap<String, usize> = HashMap::new();
        for token in tokens { *tf.entry(token.clone()).or_insert(0) += 1; }

        let mut vec: HashMap<String, f64> = HashMap::new();
        let mut norm_sq = 0.0;
        for (token, count) in tf {
            let val = (count as f64) * idf.get(&token).unwrap_or(&1.0);
            vec.insert(token, val);
            norm_sq += val * val;
        }

        let norm = norm_sq.sqrt();
        if norm > 0.0 {
            for val in vec.values_mut() { *val /= norm; }
        }
        vectors.push(vec);
    }

    for i in 0..names.len() {
        for j in (i + 1)..names.len() {
            let mut score = 0.0;
            for (token, val_i) in &vectors[i] {
                if let Some(val_j) = vectors[j].get(token) { score += val_i * val_j; }
            }
            if score > 0.95 {
                validator.fail(format!("Duplicate skill detected between '{}' and '{}' (similarity {:.2})", names[i], names[j], score));
            }
        }
    }
}

fn validate_skill_file(validator: &mut Validator, path_str: &str, raw: &str) -> Option<String> {
    println!("INFO: Validating {}", path_str);
    
    let post = match parse_frontmatter(raw) {
        Ok(p) => p,
        Err(e) => {
            validator.fail(format!("{}: {}", path_str, e));
            return None;
        }
    };

    validate_frontmatter(validator, &post.metadata, path_str);
    validate_sections(validator, &post.content, path_str);
    
    scan_patterns(validator, raw, path_str, &API_KEY_PATTERNS, "Potential API key or secret detected");
    scan_patterns(validator, raw, path_str, &SUSPICIOUS_PATTERNS, "Suspicious/Malicious shell command detected");
    scan_patterns(validator, raw, path_str, &SUSPICIOUS_INSTRUCTIONS, "Prompt Injection / Jailbreak instruction detected");
    scan_patterns(validator, raw, path_str, &OBFUSCATION_PATTERNS, "Possible obfuscated payload detected");

    let has_anti = raw.contains("**❌ Anti-pattern:**") || raw.contains("Anti-pattern") || raw.contains("### Anti-pattern");
    let has_correct = raw.contains("**✅ Correct pattern:**") || raw.contains("Correct pattern") || raw.contains("### Correct pattern");

    if !has_anti { validator.fail(format!("{}: Missing anti-pattern example in Example section", path_str)); }
    if !has_correct { validator.fail(format!("{}: Missing correct-pattern example in Example section", path_str)); }

    Some(post.content)
}

fn main() {
    let mut validator = Validator::new();
    let client = Client::new();

    let github_token = env::var("GITHUB_TOKEN").unwrap_or_default();
    let pr_number = env::var("PR_NUMBER").unwrap_or_default();
    let repo_name = env::var("REPO_NAME").unwrap_or_default();

    validate_pr_template(&mut validator, &client, &github_token, &repo_name, &pr_number);
    let changed_files = get_pr_changed_files(&client, &github_token, &repo_name, &pr_number);
    
    let mut skill_texts: HashMap<String, String> = HashMap::new();
    
    // Dynamically look for the skills directory relative to where the binary executes
    let mut skills_dir = Path::new("skills").to_path_buf();
    if !skills_dir.exists() {
        skills_dir = Path::new("../../skills").to_path_buf();
    }
    if !skills_dir.exists() {
        skills_dir = Path::new("../skills").to_path_buf();
    }

    if skills_dir.exists() {
        println!("INFO: Found 'skills' directory at: {:?}", skills_dir);
        for entry in WalkDir::new(&skills_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.file_name().unwrap_or_default() == "SKILL.md" {
                // Keep file path mapping friendly to matching PR diff output strings
                let absolute_str = path.to_string_lossy().replace("\\", "/");
                let relative_path = if absolute_str.contains("skills/") {
                    format!("skills/{}", absolute_str.splitn(2, "skills/").collect::<Vec<&str>>()[1])
                } else {
                    absolute_str
                };
                
                let is_changed = changed_files.is_empty() || changed_files.iter().any(|f| relative_path.ends_with(f) || f.ends_with(&relative_path));

                if let Ok(raw) = fs::read_to_string(path) {
                    if is_changed {
                        if let Some(content) = validate_skill_file(&mut validator, &relative_path, &raw) {
                            skill_texts.insert(relative_path, content);
                        }
                    } else if let Ok(post) = parse_frontmatter(&raw) {
                        skill_texts.insert(relative_path, post.content);
                    }
                }
            }
        }
    } else {
        println!("ERROR: 'skills' directory could not be resolved! Looked in root, ../, and ../../");
        validator.fail("Repository skills directory directory was completely missing or unreadable.".into());
    }

    detect_duplicates(&mut validator, &skill_texts);

    println!("\n================ VALIDATION REPORT ================\n");

    for warning in &validator.warnings {
        println!("WARNING: {}", warning);
    }

    if !validator.errors.is_empty() {
        for error in &validator.errors {
            println!("ERROR: {}", error);
        }
        println!("\nValidation failed.");
        process::exit(1);
    }

    println!("All automated checks passed.");
    println!("Human review is still REQUIRED before merge.");
    process::exit(0);
}
