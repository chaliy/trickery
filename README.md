# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

cli to generate textual artifacts using LLM.

Idea is simple, imagine you need to generate some docs using LLM as part of CI, this is a tool for you.

> [!TIP]
> This README was generated with trickery
> trickery generate -i ./prompts/trickery_readme.md > README.md

## How to install

If you have rust/cargo installed, you can install `trickery` with:

```sh
cargo install --git https://github.com/chaliy/trickery.git
trickery --help
```

## How to use

```sh
export OPENAI_API_KEY=s....d
trickery generate -i ./prompts/trickery_readme.md > README.md
```

Input file could be any text file, with Jinja2-like template variables, like `{{app_version}}`. To set this variables, please use `-v` flag, like `-v app_version=1.0.0`.

## Українською 🇺🇦

Trickery — це маленький інструмент командного рядка для генерації текстових артефактів за допомогою великих мовних моделей (LLM). Його ідея дуже проста: якщо вам потрібно автоматично згенерувати документацію, реліз-ноти або інші текстові файли в процесі CI/CD, trickery дозволяє зробити це легко, підставивши змінні в шаблонах та викликавши модель для генерації вмісту. Інструмент орієнтований на простоту використання та інтеграцію в існуючі скрипти і пайплайни.

- Підтримувані шаблони: будь-які текстові файли з Jinja2-подібними змінними (`{{name}}`)
- Налаштування через параметри `-v` (наприклад `-v app_version=1.0.0`)
- Працює добре в CI: можна експортувати ключі та викликати з консолі

Будь ласка, повідомляйте про баги та пропозиції в репозиторії, будемо раді внеску.

## Dad Joke

Why did the developer go broke? Because he used up all his cache.

