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
- Uses llm-chain for LLM integration with OpenAI
- Supports Jinja2-like template variables in prompts
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
│   └── generate.rs   # Generate command implementation
└── trickery/
    ├── mod.rs
    └── generate.rs   # LLM template generation logic
prompts/              # Example prompt templates
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

PR titles should follow Conventional Commits format.
