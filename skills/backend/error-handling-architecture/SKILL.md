---
name: error-handling-architecture
description: When designing how a system recovers from and reports failures.
version: 1.0.0
tags: [backend, errors, resilience]
---

# Error Handling Architecture

## When to use
- Setting up a new backend service.
- Refactoring code with excessive `try/catch` blocks.
- Standardizing API error responses.

## What it does
Creates a central, robust pipeline for capturing, categorizing, logging, and responding to application exceptions without leaking sensitive data.

## Workflow
1. **Categorize Errors**: Distinguish between Operational Errors (e.g., network failure, bad user input) and Programmer Errors (e.g., null pointer, memory leak).
2. **Define Custom Error Classes**: Create an `AppError` base class with properties for HTTP status code and an "isOperational" flag.
3. **Centralize Middleware**: Catch all exceptions at the top level of the framework (e.g., Express error middleware).
4. **Log Appropriately**: Log full stack traces for 5xx errors; log only messages and metadata for 4xx errors.
5. **Sanitize Output**: Strip stack traces and sensitive database messages before sending the response to the client.

## Rules
- Never silently swallow exceptions (`catch(e) {}`).
- Crash and restart the process for unhandled Programmer Errors.
- All HTTP error responses must follow a standard JSON schema.

## Anti-patterns
- **Throwing Strings**: `throw "User not found"` (always throw Error objects).
- **Leaking DB Details**: Sending raw SQL constraint violation messages to the client.

## Output format
A centralized error handler function/middleware and a structured JSON error response format.
