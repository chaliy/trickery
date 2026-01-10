# Coding-agent guidance (repo root)

This repo is intended to be runnable locally and easy for coding agents to work in.

Style Telegraph. Drop filler/grammar. Min tokens (global AGENTS + replies).

Critical Thinking Fix root cause (not band-aid). Unsure: read more code; if still stuck, ask w/ short options. Unrecognized changes: assume other agent; keep going; focus your changes. If it causes issues, stop + ask user. Leave breadcrumb notes in thread.

## Principles

- Keep decisions as comments on top of the file. Only important decisions that could not be inferred from code.
- Code should be easily testable, smoke testable, runnable in local dev env.
- Prefer small, incremental PR-sized changes with a runnable state at each step.
- Avoid adding dependencies with non-permissive licenses. If a dependency is non-permissive or unclear, stop and ask the repo owner.

## Top level requirements

- CLI tool for generating textual artifacts using LLM
- Minimal self-contained OpenAI provider (no external LLM libraries)
- Supports Jinja2-like template variables in prompts
- Model selection and reasoning level configuration
- Designed for CI/CD integration

## Local dev expectations

- Rust toolchain (stable)
- `cargo build` to build
- `cargo test` to run tests
- `cargo fmt` to format code
- `cargo clippy` for linting
- `OPENAI_API_KEY` environment variable required for runtime

## Code organization

```
src/
├── main.rs           # CLI entry point, clap argument parsing
├── output.rs         # JSON output utilities
├── commands/
│   ├── mod.rs        # Command traits (CommandExec, CommandResult)
│   ├── generate.rs   # Generate command implementation
│   └── image.rs      # Image generation command implementation
├── provider/
│   ├── mod.rs        # Provider abstraction types (Chat + Responses API)
│   └── openai.rs     # OpenAI provider implementation
└── trickery/
    ├── mod.rs
    ├── generate.rs   # LLM template generation logic
    └── image.rs      # Image generation logic
prompts/              # Example prompt templates
test_cases/           # Test case templates for generate command
specs/                # Feature specifications
docs/                 # Feature documentation
```

## Naming

- Use snake_case for files and functions
- Use PascalCase for types and traits
- Keep module names short and descriptive

## CI expectations

CI is implemented using GitHub Actions (`.github/workflows/ci.yaml`):
- Runs on push/PR to main
- Executes `cargo build --verbose`
- Executes `cargo test --verbose`

## Pre-PR checklist

1. Formatting: Run `cargo fmt`
2. Linting: Run `cargo clippy` and fix warnings
3. Tests: Ensure `cargo test` passes
4. Build: Ensure `cargo build` succeeds

## Commit message conventions

Follow Conventional Commits format:
- `feat:` new feature
- `fix:` bug fix
- `docs:` documentation changes
- `style:` formatting, no code change
- `refactor:` code restructuring
- `perf:` performance improvements
- `test:` adding/updating tests
- `chore:` maintenance tasks
- `ci:` CI configuration changes

## PR conventions

PR titles should follow Conventional Commits format: `<type>[optional scope]: <description>`

### PR body template

```markdown
## What
Clear description of the change.

## Why
Problem or motivation.

## How
High-level approach.

## Risk
- Low / Medium / High
- What can break

### Checklist
- [ ] Unit tests are passed
- [ ] Smoke tests are passed
- [ ] Documentation is updated
```

## Specs

`specs/` folder contains feature specifications outlining requirements for specific features and components. New code should comply with these specifications or propose changes to them.

Available specs:

- `llm-provider.md` - LLM provider abstraction, OpenAI integration, design choices

Specification format: Abstract and Requirements sections.

## Test Cases

`test_cases/` folder contains manual smoke test cases for validating CLI functionality. Run these after changes to verify behavior.

Available test cases:

- `basic_generation.md` - Simple prompt generation without variables
- `template_variables.md` - Jinja2-style variable substitution
- `json_output.md` - JSON output format flag
- `image_multimodal.md` - Image input for multimodal prompts
- `image_generate.md` - Image generation and editing command
- `error_handling.md` - Error scenarios and messages

### Test case template

```markdown
# Test: <Name>

## Abstract
<One sentence describing what this test validates>

## Prerequisites
- `cargo install --path .`
- <Other required setup, env vars, files>

## Steps

### 1. <Step name>
**Run:** `trickery <command>`
**Expect:** <Expected outcome>
```
