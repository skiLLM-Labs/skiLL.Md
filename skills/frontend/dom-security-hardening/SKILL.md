---
name: dom-security-hardening
description: When hardening a web application against Cross-Site Scripting (XSS) and injection attacks.
version: 2.0.0
category: hacking
tags: [frontend, security, xss, csp]
skill_type: security
author: skiLLM
license: MIT
compatible_agents: [claude-code, cursor, copilot, codex]
estimated_context_tokens: 2200
dangerous: false
requires_review: true
security_level: dangerous
dependencies: [react-component-design]
triggers: [xss, injection, dom manipulation, csp, security, innerHTML, dangerouslySetInnerHTML]
permissions:
  filesystem: { read: true, write: true }
  network: { outbound: false }
  shell: { execute: false }
input_requirements: [HTML codebase, JavaScript with DOM manipulation]
output_contract: [no inline scripts, no inline styles, csp enforced, no unsafe dom apis]
failure_conditions: [required sanitizer library unavailable, csp cannot be enforced, legacy framework requires unsafe-inline]
last_updated: 2026-05-15
---

# DOM Security Hardening

## Purpose
XSS attacks kill applications. This skill hardens the DOM attack surface by enforcing a strict Content Security Policy, eliminating unsafe DOM APIs, and stripping execution vectors like inline scripts and styles. Following this skill is MANDATORY for any user-facing web application handling user input.

## When to use
- Setting up the initial `index.html` or document root of a web application
- Refactoring legacy code that relies on direct DOM manipulation
- Auditing a frontend codebase for XSS vulnerabilities
- Processing rich text or markdown input from untrusted users
- Creating forms or chat systems that accept user input

## When NOT to use
- CSS framework design (separate concern)
- React component prop validation (use React Component Design)
- Backend input validation (different layer, still required)

## Inputs required
- HTML document structure
- JavaScript files with DOM manipulation
- Understanding of Content Security Policy basics
- Sanitizer library decision (DOMPurify, sanitize-html, etc.)

## Workflow
1. **Enforce CSP**: Add strict `<meta http-equiv="Content-Security-Policy">` tag in HTML `<head>` (or HTTP headers)
2. **Externalize Assets**: Extract ALL inline `<script>` and `<style>` blocks into separate `.js` and `.css` files
3. **Remove Inline Events**: Replace inline `onclick="..."`, `onchange="..."` with standard `addEventListener` bindings
4. **Replace Unsafe DOM**: Replace `innerHTML`, `outerHTML`, `dangerouslySetInnerHTML` with `textContent` or `innerText`
5. **Implement Sanitizer**: If HTML rendering is REQUIRED, use DOMPurify or equivalent BEFORE insertion
6. **Test CSP**: Verify policy blocks all unauthorized execution attempts
7. **Verify No Bypasses**: Use security scanner (OWASP ZAP, Burp) to confirm XSS vectors are closed

## Rules
- MUST implement strict CSP: `default-src 'self'; script-src 'self'; style-src 'self'; object-src 'none'`
- MUST NEVER use inline `<script>` blocks in HTML
- MUST NEVER use inline `style="..."` attributes
- MUST NEVER use `eval()`, `setTimeout(string)`, `new Function(string)`
- MUST NEVER use `innerHTML`, `outerHTML`, or `dangerouslySetInnerHTML` with user input
- MUST use `textContent` or `innerText` for all dynamic text insertions
- MUST sanitize user HTML with DOMPurify before any insertion
- MUST NOT use `'unsafe-inline'` or `'unsafe-eval'` in CSP

## Anti-patterns
- **`innerHTML` Assignment**: `element.innerHTML = userInput` (immediate XSS)
- **`javascript:` URIs**: `href="javascript:void(0)"` or `href="javascript:alert(1)"`
- **Unsafe CSP**: `Content-Security-Policy: default-src *; script-src 'unsafe-inline'`
- **Event Handler Strings**: Creating event handlers from user input or strings
- **Trusting User Input**: Assuming any user input is safe to insert into DOM
- **Missing Sanitizer**: Using rich text editor without sanitizing output

## Failure conditions
- CSP header/meta tag is missing
- Inline scripts or styles remain in production code
- `innerHTML` used with user input
- `dangerouslySetInnerHTML` used without sanitization
- CSP allows `'unsafe-inline'` or `'unsafe-eval'`
- Sanitizer library is not installed for rich text rendering

## Validation checklist
- [ ] CSP header/meta tag present with restrictive policy
- [ ] No `<script>` tags with inline code (all external)
- [ ] No inline `style="..."` attributes (all CSS classes)
- [ ] No `onclick`, `onchange`, `oninput` inline event handlers
- [ ] All DOM text insertions use `textContent` or `innerText`
- [ ] No `innerHTML`, `outerHTML`, or `dangerouslySetInnerHTML` with user input
- [ ] DOMPurify (or equivalent) used for any rich text rendering
- [ ] No `eval()`, `setTimeout(string)`, `new Function(string)` calls
- [ ] Security scanner passes (no XSS vulnerabilities detected)
- [ ] CSP blocks inline script execution (verify in browser console)

## Output format
- **HTML file**: Strict CSP meta tag in `<head>`, external `<script>` tags at end of `<body>`
- **JavaScript**: All DOM mutations via safe APIs (`textContent`, `className`, `setAttribute`)
- **CSS**: Separate `.css` files, no inline styles anywhere
- **Rich Text**: HTML sanitized via DOMPurify before insertion
- **Validation**: Automated XSS scan passes

## Agent execution notes
- Agent MAY: Add CSP header, externalize inline scripts/styles, replace innerHTML with textContent, implement DOMPurify
- Agent MUST NEVER: Use `'unsafe-inline'` or `'unsafe-eval'` in CSP, ignore sanitization requirements, leave inline event handlers
- Agent MUST ASK: Before adding third-party scripts, before relaxing CSP for legacy code, before using dangerouslySetInnerHTML
- Agent MUST VALIDATE: CSP policy is strict, no inline scripts remain, no unsafe DOM APIs, XSS scanner passes

## Example

**❌ Anti-pattern (Unsafe XSS vectors, no CSP):**
```html
<!-- No CSP -->
<a href="javascript:void(0)" onclick="submitForm()">Click me</a>
<div id="user-bio"></div>

<script>
  // Unsafe: direct user input to DOM
  document.getElementById('user-bio').innerHTML = getUserInput();
  
  // Unsafe: inline style
  document.getElementById('user-bio').setAttribute('style', 'color: red;');
  
  // Unsafe: event handler string
  setTimeout("console.log('vulnerable')", 1000);
</script>
```

**✅ Correct pattern (Hardened, safe):**
```html
<!-- Strict CSP enforced -->
<meta http-equiv="Content-Security-Policy"
  content="default-src 'self'; script-src 'self'; style-src 'self'; object-src 'none'; img-src 'self' https:; font-src 'self';"
>

<a href="#" id="submit-btn">Click me</a>
<div id="user-bio"></div>

<!-- External script only -->
<script src="/js/app.js"></script>
```

```javascript
// app.js

// 1. Safe event binding
document.getElementById('submit-btn').addEventListener('click', (e) => {
  e.preventDefault();
  submitForm();
});

// 2. Safe text insertion (escapes HTML by default)
document.getElementById('user-bio').textContent = getUserInput();

// 3. Safe styling via CSS classes
document.getElementById('user-bio').classList.add('text-red');

// 4. If HTML rendering is absolutely required, sanitize first
import DOMPurify from 'dompurify';
document.getElementById('user-bio').innerHTML = DOMPurify.sanitize(getUserMarkdown());
```

## Run Command
```bash
curl -fsSL https://claude.ai/install.sh | bash
```
