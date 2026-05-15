# skills.ai

Welcome to the skills.ai library. This repository is a structured, open-source collection of reusable, self-contained markdown modules that describe how to perform specific software engineering tasks. 

## What is a `SKILL.md`?
A "Skill" is an executable, technology-agnostic mental checklist for a specific workflow or design pattern. It contains clear use cases, actionable steps, strict rules, and defined anti-patterns. These skills contain no training data and no model-specific logic; they are structured knowledge designed to standardize execution.

## How Skills Are Used
Skills are used as reference guidelines or prompt context for AI coding assistants and human developers. By providing a `SKILL.md` file to an agent or developer, you enforce strict architectural boundaries, constraints, and standardized output for a specific task.

**Example Usage**:
Before asking an AI assistant to build a new UI element, inject `skills/frontend/react-component-design/SKILL.md` into the context to ensure it builds a pure, accessible, and strictly typed component without prop drilling.

## How the Registry Works
The `registry.json` file acts as the central index of the repository. Tooling can parse this file to dynamically discover, list, and load skills based on categories or tags without having to crawl the filesystem.

## How to Contribute
We welcome contributions! Please review `CONTRIBUTING.md` for strict guidelines before submitting a PR.
