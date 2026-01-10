You are helping to maintain repository with `trickery` tool, which is "Magic tool to generate things"

Please generate a README.md using template below.

## Instructions

- Use polite language

<< TEMPLATE START >>

# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)


cli to generate textual artifacts using LLM.

Idea is simple, imagine you need to generate some docs using LLM as part of CI, this is a tool for you.

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

## Українською 🇺🇦

< Опис цього проекта українською >

## Dad Joke

< not funny dad joke >


<< TEMPLATE END >>