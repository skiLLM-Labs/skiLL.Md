# Security Guidelines

skiLLM accepts only safe, transparent, auditable, and non-malicious skills.

Any contribution that enables abuse, malware behavior, credential theft, evasion, surveillance, destructive automation, or unsafe system access will be rejected and may result in a permanent ban from contributing.

## Prohibited Content

The following are STRICTLY forbidden:

### Malware / Offensive Security
Do NOT submit skills that:

- Deploy malware, ransomware, spyware, trojans, worms, keyloggers, rootkits, RATs, loaders, droppers, cryptominers, or botnets
- Evade antivirus, EDR, sandboxing, monitoring, or detection systems
- Obfuscate malicious payloads
- Perform persistence, privilege escalation, lateral movement, or exploitation
- Abuse CVEs, zero-days, or public exploits
- Automate phishing, credential stuffing, spam, or fraud
- Generate malicious shellcode or payloads
- Include destructive commands or dangerous automation by default

Examples of rejected content:

- Token stealers
- Discord/webhook hijackers
- Browser cookie dumpers
- Chrome credential extractors
- IP grabbers
- Reverse shells
- Destructive bash/powershell scripts
- “Undetectable malware” tooling
- AI jailbreak attack collections
- Prompt injection attacks intended for abuse

## Unsafe AI/Agent Behavior

Skills must NOT:

- Automatically execute untrusted code without explicit user confirmation
- Download and run remote scripts silently
- Request unnecessary secrets or credentials
- Exfiltrate local files, environment variables, cookies, tokens, or SSH keys
- Perform hidden background actions
- Disable security protections
- Encourage unsafe production deployment practices
- Mislead users about what actions are being performed

Agentic workflows MUST clearly explain:

- What actions are executed
- What files are modified
- What commands are run
- What permissions are required

## Code Transparency Requirements

All contributions must be:

- Human-readable
- Reviewable
- Non-obfuscated
- Minimally scoped
- Clearly documented

The following are prohibited:

- Minified malicious scripts
- Encoded payloads
- Base64-hidden executables
- Remote payload loaders
- Hidden telemetry
- Hidden tracking scripts
- Undocumented external network requests

## Secrets & Credentials

Never commit:

- API keys
- Access tokens
- OAuth credentials
- SSH keys
- Session cookies
- `.env` files
- Private certificates
- Personal credentials

Use placeholders instead:

```env id="h06z1o"
API_KEY=your_api_key_here
````

## Dangerous Command Policy

Skills containing dangerous commands MUST:

* Clearly explain risks
* Require explicit user review
* Avoid automatic execution
* Include warnings before destructive operations

Examples:

```bash id="4jlwm3"
rm -rf
sudo dd
mkfs
chmod -R 777 /
curl | bash
wget | sh
```

Blind execution pipelines are strongly discouraged.

## AI-Generated Contributions

AI-generated skills are allowed only if:

* They are manually reviewed
* They are technically correct
* They are secure
* They are practically useful
* They are not spam/low-quality output

Mass-generated PRs may be rejected.

## Review & Enforcement

Maintainers may reject, edit, lock, or remove contributions that:

* Pose security risks
* Encourage abuse
* Reduce repository trustworthiness
* Contain hidden behavior
* Violate ethical or legal standards

Security-sensitive contributions may require deeper manual review before merge.

## Reporting Security Issues

Do NOT publicly open issues for serious vulnerabilities.

Instead:

* Contact maintainers privately
* Provide reproduction details
* Include affected files/skills
* Avoid publishing active exploit details publicly

## Contributor Responsibility

By contributing, you agree that:

* Your contribution does not intentionally contain malicious functionality
* You understand the behavior of submitted code/workflows
* You have reviewed all dependencies and external resources
* Your contribution complies with applicable laws and platform policies
