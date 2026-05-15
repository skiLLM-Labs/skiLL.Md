<div align="center">

<img height="125" alt="ChatGPT Image May 15, 2026, 02_52_01 PM" src="https://github.com/user-attachments/assets/b83e5c3a-d8c5-4c58-b2b7-bff31adabcaf" />

**skills.ai is a structured, open-source collection of reusable, self-contained Markdown modules designed to teach AI coding assistants and human developers how to perform specific software engineering tasks.**

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)]()
[![Markdown Standard](https://img.shields.io/badge/Markdown-Strict-black.svg)]()
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)]()
<br />
[![Total Skills](https://img.shields.io/badge/Total_Skills-10-orange.svg)]()
[![Schema](https://img.shields.io/badge/Schema-Valid-blueviolet.svg)]()
[![Registry](https://img.shields.io/badge/Registry-Synchronized-success.svg)]()
[![Standard](https://img.shields.io/badge/Standard-SOP--Ready-yellow.svg)]()
<br />
[![Frontend](https://img.shields.io/badge/Category-Frontend-ff69b4.svg)]()
[![Backend](https://img.shields.io/badge/Category-Backend-6b4fbb.svg)]()
[![Architecture](https://img.shields.io/badge/Category-Architecture-007acc.svg)]()
[![DevTools](https://img.shields.io/badge/Category-DevTools-2ea44f.svg)]()

<p align="center">
  <a href="#how-to-use-the-library">Usage Guide</a> •
  <a href="#repository-architecture">Architecture</a> •
  <a href="./CONTRIBUTING.md">Contributing</a>
</p>

</div>

## The Philosophy: Why Does This Exist?

When working with modern AI coding assistants (like Cursor, GitHub Copilot, or Claude) or autonomous agents, the quality of the output is heavily dependent on the context provided. 

Massive, monolithic "system prompts" or `.cursorrules` files quickly become unmanageable, leading to token bloat and diluted AI focus. 

**The skills.ai solves this by introducing modularity.**
Instead of telling the AI *everything* about your codebase at once, you dynamically inject hyper-specific, technology-agnostic checklists (`SKILL.md`) exactly when the AI needs them. 

- **No generic advice** ("Write clean code").
- **No training datasets** (Only rules, constraints, and actionable workflows).
- **No model-specific logic** (Works across GPT-5.5, Claude 4.7 Opus, Gemini 3.1 Pro, etc.).

## What is a `SKILL.md`?

A "Skill" is an executable, technology-agnostic mental checklist for a specific workflow, design pattern, or architecture. Every skill in this repository strictly adheres to `schema/SKILL_SCHEMA.md` and contains:

1. **Clear Use Cases:** Exact conditions for when the skill should be activated.
2. **Actionable Workflows:** Step-by-step instructions.
3. **Hard Constraints:** Non-negotiable architectural rules.
4. **Anti-patterns:** Specific mistakes the AI (or developer) must avoid.
5. **Output Formats:** Expected code footprint.

## How to Use the Library

Skills are meant to be used as injected context. Here is how you can use them across different workflows:

### 1. With AI Code Editors (Cursor, Windsurf, Copilot)
When asking your IDE's chat or inline generator to perform a task, `@`-mention the relevant skill file to strictly bind the AI's output to the standard.
> **Prompt:** *"I need to build a new Settings page for the user dashboard. Please review `@skills/frontend/react-component-design/SKILL.md` and `@skills/ui-ux/accessibility-ui-design/SKILL.md` before generating the code."*

### 2. With Autonomous Agents / Custom GPTs
If you are building a custom AI agent, use the `registry.json` file. 
Provide the registry to your agent as a tool. When the user asks for a task, the agent can query the registry, find the path to the required `SKILL.md`, and read the file into its context window *before* writing code.

### 3. As Human SOPs (Standard Operating Procedures)
Because every skill is written in clear, checklist-style Markdown, they double perfectly as onboarding documents, PR review checklists, and team engineering standards.

## Repository Architecture

The library is organized by domain, ensuring skills are easy to find and categorize.

```text
skills-library/
├── README.md                 # You are here
├── CONTRIBUTING.md           # Strict rules for adding new skills
├── registry.json             # Central index of all skills for programmatic discovery
├── schema/
│   └── SKILL_SCHEMA.md       # The mandatory template for all SKILL.md files
└── skills/                   # The core skill repository
    ├── frontend/             # React, UI patterns, state management
    ├── backend/              # API design, database, error handling
    ├── dev-tools/            # Git workflows, CLI, debugging
    ├── ui-ux/                # A11y, design systems, animation
    ├── architecture/         # Caching, event-driven, scalability
    └── code-quality/         # Testing, PR reviews, refactoring
```

### The Power of `registry.json`
The repository includes an auto-updated `registry.json`. This acts as an API for the filesystem. Tooling, CI/CD pipelines, and AI agents can parse this file to dynamically load skills by `category` or `tags` without needing to crawl directories.

## Concrete Example: Prompt Injection

Here is an example of what happens when you combine an AI prompt with a Skill from this library.

**Without the Skill:**
> *"Write a generic React button component."*
> **Result:** The AI might write a class component, it might use inline styles, it might forget ARIA labels, or it might overcomplicate the props.

**With the Skill:**
> *"Write a React button component. Adhere strictly to the rules and anti-patterns defined in the attached `react-component-design/SKILL.md`."*
> **Result:** The AI outputs a pure functional component, strictly typed via TypeScript interfaces, isolated from business logic, utilizing CSS classes instead of inline styles, and avoiding prop-drilling.

## Contributing

We welcome contributions from the community! However, to maintain the high quality and uniformity of this library, **all contributions must pass strict quality checks.**

Please read the [CONTRIBUTING.md](./CONTRIBUTING.md) before submitting a Pull Request.

**Golden Rules of Contributing:**
1. **One Skill = One Folder:** E.g., `skills/domain/my-new-skill/SKILL.md`.
2. **Follow the Schema:** Your submission will be rejected if it does not match `schema/SKILL_SCHEMA.md`.
3. **No Vague Advice:** "Write readable code" is not a skill. "Limit functions to 3 arguments and return a single typed object" is a skill.
4. **Update the Registry:** Don't forget to add your new skill to `registry.json`.

If you aren't sure if your skill belongs here, open a **New Skill Proposal** Issue using our issue templates!
