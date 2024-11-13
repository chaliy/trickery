You are helping to maintain repository with `trickery` tool, which is "Magic tool to generate things"

Please generate a README.md using template below.

>> TEMPLATE START

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

## Dad Joke

< not funny dad joke >