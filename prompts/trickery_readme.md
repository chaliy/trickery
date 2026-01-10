You are helping to maintain repository with `trickery` tool, which is "Coding Agent friendly tool to magically generate text and images"

Please generate a README.md using template below.

## Instructions

- Use polite language

<< TEMPLATE START >>

# Trickery

Coding Agent friendly tool to magically generate text and images.

[![CI](https://github.com/chaliy/trickery/actions/workflows/ci.yaml/badge.svg)](https://github.com/chaliy/trickery/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/trickery)](https://crates.io/crates/trickery)
[![Coding Agent Friendly](https://img.shields.io/badge/coding%20agent-friendly-brightgreen)](specs/coding-agent-design.md)

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

CLI for generating textual and visual artifacts using LLM. Designed for CI/CD pipelines and AI coding agents.

Idea is simple: need to generate docs, images, or other artifacts as part of CI? This tool integrates seamlessly into scripts and agent workflows.

> [!TIP]
> This README was generated with trickery
> trickery generate ./prompts/trickery_readme.md > README.md


## Demo

![Demo Screenshot](/docs/images/images.png)

## How to install

If you have rust/cargo installed, you can install `trickery` with:

```sh
cargo install trickery
trickery --help
```

## How to use

```sh
export OPENAI_API_KEY=s....d
trickery generate ./prompts/trickery_readme.md > README.md
```

Input file could be any text file, with Jinja2-like template variables, like `{{"{{app_version}}"}}`. To set this variables, please use `-v` flag, like `-v app_version=1.0.0`.

## Documentation

- [Input Images](docs/input-images.md) - Using images in multimodal prompts
- [Image Generation](docs/image-generation.md) - Generating and editing images

## Agent-Friendly Design

Trickery is built with AI coding agents in mind:

- **Rich error messages** - Errors include context and recovery hints, so agents can self-correct
- **Full help system** - Run `trickery help --full` for comprehensive documentation with examples
- **Predictable output** - Use `--json` for structured output that's easy to parse
- **Template variables** - Reproducible prompts with `{{ variable }}` substitution
- **Auto-detection** - Input can be file path or direct text, no flags needed
- **Exit codes** - Proper exit codes for script/agent error handling

## Українською 🇺🇦

< Опис цього проекта українською >

## Dad Joke

< not funny dad joke >


<< TEMPLATE END >>