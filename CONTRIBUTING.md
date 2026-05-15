# Contributing Guidelines

To maintain the high quality and uniformity of this skills library, all contributions must adhere to the following strict rules:

1. **One Skill = One Folder**: Every skill must live in its own directory under the appropriate category inside `skills/`.
2. **Must Include `SKILL.md`**: The core file must be named `SKILL.md` and placed at the root of the skill folder. Optional files like `examples.md` or `references.md` are permitted.
3. **Must Follow the Schema**: The `SKILL.md` file must strictly adhere to `schema/SKILL_SCHEMA.md`, including frontmatter and exact header names.
4. **Define Usage Conditions**: You must clearly state the exact conditions for activation in the "When to use" section.
5. **No Overlap**: Your skill must cover a unique pattern. It must not overlap with existing skills in the repository.
6. **Manually Testable**: Every skill's workflow must act as a mental checklist that a human developer can manually test and execute.
7. **No Vague Advice**: "Write clean code" is not a skill. Provide concrete, actionable rules and hard constraints.
