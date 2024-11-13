# Magic tool to generate things

[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

`trickery` is a CLI tool to generate textual artifacts using LLM.

The idea is simple: imagine you need to generate some documentation as part of your CI process; this is the tool for you.

> [!TIP]
> This README was generated with trickery
> `trickery generate -i ./prompts/trickery_readme.md > README.md`

## How to install

If you have Rust and Cargo installed, you can easily install `trickery` with:

```bash
cargo install --git https://github.com/chaliy/trickery.git
trickery --help
```

### ZSH

To enable autocomplete for `trickery` in ZSH, you can run:

```bash
trickery completion zsh > $ZSH/cache/completions/_trickery
compinit
trickery <TAB>
```

## Dad Joke

Why don't skeletons fight each other? They don't have the guts!
