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
cargo install trickery
trickery --help
```

## How to use

```sh
export OPENAI_API_KEY=s....d
trickery generate -i ./prompts/trickery_readme.md > README.md
```

Input file could be any text file, with Jinja2-like template variables, like `{{"{{app_version}}"}}`. To set this variables, please use `-v` flag, like `-v app_version=1.0.0`.

## Українською 🇺🇦

Цей інструмент дозволяє зручно та автоматично генерувати текстові артефакти (документацію, описові файли тощо) за допомогою великих мовних моделей. Ідея проста: інтегруйте генерацію у ваш CI/CD або виконуйте локально, підставляючи змінні у шаблони. Будь ласка, використовуйте інструмент відповідально та перевіряйте згенерований контент перед публікацією.

## Dad Joke

Why did the developer bring a ladder to work? Because they heard the code needed to be taken to the next level.
