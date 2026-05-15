---
name: debugging-strategies
description: When isolating and fixing unpredictable or complex software bugs.
version: 1.0.0
tags: [dev-tools, debugging, troubleshooting]
---

# Debugging Strategies

## When to use
- Investigating a defect reported in production.
- Facing a failing test with no obvious root cause.
- Understanding unfamiliar, undocumented legacy code.

## What it does
Replaces "guess and check" coding with a systematic scientific method to isolate the root cause of software defects.

## Workflow
1. **Reproduce the Bug**: Consistently replicate the failure state. If you can't reproduce it, you can't fix it. Write a failing test.
2. **Isolate the Subsystem**: Use binary search (commenting out code halves or using `git bisect`) to locate the exact module causing the issue.
3. **Formulate a Hypothesis**: Propose *why* the failure is happening based on logs and stack traces.
4. **Test the Hypothesis**: Add targeted logging or use a step-through debugger to verify assumptions about state/variables at runtime.
5. **Apply the Fix & Verify**: Apply the minimal change required to fix the issue. Verify the failing test now passes.

## Rules
- Always establish a feedback loop (a reproducible test) before changing code.
- Only change one variable or line of code at a time during the testing phase.

## Anti-patterns
- **Shotgun Debugging**: Randomly changing configuration or code until things "seem to work."
- **Blaming the Compiler**: Assuming the language, framework, or standard library is broken before checking your own code.

## Output format
A minimal, targeted code patch accompanied by an automated regression test.
