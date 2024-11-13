# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

CLI to generate textual artifacts using LLM.

The idea is simple: imagine you need to generate some documentation using LLM as part of CI; this is the tool for you.

> [!TIP]
> This README was generated with trickery
> `trickery generate -i ./prompts/trickery_readme.md > README.md`

## How to install

If you have Rust/Cargo installed, you can install `trickery` with:

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

Why don't skeletons fight each other? They don't have the guts!
