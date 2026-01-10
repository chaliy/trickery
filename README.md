# Trickery

Coding Agent friendly tool to magically generate text and images.

[![CI](https://github.com/chaliy/trickery/actions/workflows/ci.yaml/badge.svg)](https://github.com/chaliy/trickery/actions/workflows/ci.yaml)
[![Crates.io](https://img.shields.io/crates/v/trickery)](https://crates.io/crates/trickery)
[![Repo: Agent Friendly](https://img.shields.io/badge/repo-agent%20friendly-blue)](AGENTS.md)

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

### Using with OpenAI-compatible gateways

You can use trickery with any OpenAI-compatible API gateway (like LiteLLM, Azure OpenAI, or local models) by setting the `OPENAI_BASE_URL` environment variable:

```sh
export OPENAI_API_KEY=your-key
export OPENAI_BASE_URL=http://localhost:4000/v1
trickery generate ./prompts/my_prompt.md
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

Trickery — невеликий інструмент командного рядка для автоматичної генерації текстових артефактів за допомогою великих мовних моделей. Ідея проста: якщо вам потрібно генерувати документацію, звіти або інші тексти в рамках CI/CD, цей інструмент допоможе інтегрувати виклики LLM у ваші скрипти та конвеєри. Доступні варіанти підстановки змінних у шаблонах, можливість роботи з мультимодальними підказками та простий інтерфейс для інтеграції в існуючі процеси.

Якщо маєте питання або пропозиції — ласкаво просимо відкрити issue або pull request у репозиторії.

## Dad Joke

Why did the developer go broke? Because he used up all his cache.


