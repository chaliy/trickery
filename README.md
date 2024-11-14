# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

Welcome to the `trickery` tool, a command-line interface designed to generate textual artifacts using large language models (LLM). 

The idea behind `trickery` is simple: imagine you need to generate documentation as part of your continuous integration (CI) process. This tool is here to assist you in that endeavor.

> [!TIP]
> This README was generated with `trickery` 
> ```
> trickery generate -i ./prompts/trickery_readme.md > README.md
> ```

## How to install

If you have Rust and Cargo installed, you can easily set up `trickery` with the following command:

```sh
cargo install --git https://github.com/chaliy/trickery.git
trickery --help
```

## How to use

To generate files using `trickery`, you can follow these steps:

```sh
export OPENAI_API_KEY=s....d
trickery generate -i ./prompts/trickery_readme.md > README.md
```

The input file can be any text file that contains Jinja2-like template variables, such as `{{app_version}}`. To set these variables, you can use the `-v` flag, for example: `-v app_version=1.0.0`.

## Українською 🇺🇦

Цей інструмент допомагає автоматизувати процес генерації текстових артефактів за допомогою великих мовних моделей. Легко використовувати та інтегрувати у ваш CI процес.

## Dad Joke

Why did the scarecrow win an award? Because he was outstanding in his field!
