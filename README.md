# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)


cli to generate textual artifacts using LLM.

Idea is simple, imagine you need to generate some docs using LLM as part of CI, this is a tool for you.

> [!TIP]
> This README was generated with trickery
> trickery generate -i ./prompts/trickery_readme.md > README.md

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
trickery generate -i ./prompts/trickery_readme.md > README.md
```

Input file could be any text file, with Jinja2-like template variables, like `{{"{{app_version}}"}}`. To set this variables, please use `-v` flag, like `-v app_version=1.0.0`.

## Українською 🇺🇦

trickery — це невеликий інструмент командного рядка для автоматичної генерації текстових артефактів за допомогою LLM. Ідея полягає в тому, щоб інтегрувати генерацію документації чи інших текстів у CI/CD або виконувати її локально з шаблонів. Ви можете підготувати файл-запит із місцями для підстановки (шаблони в стилі Jinja2), передавати значення змінних через прапорець `-v` і отримувати готові артефакти у stdout або в файл. Дякуємо, що користуєтеся trickery — будь ласка, відкривайте issue або PR, якщо побачите ідеї для покращення.

## Dad Joke

Why did the scarecrow become a successful software engineer? Because he was outstanding in his field.
