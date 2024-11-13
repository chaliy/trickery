# Magic tool to generate things

CLI to generate textual artifacts using LLM.

Idea is simple, imagine you need to generate some docs as part of CI, this is a tool for you.

> [!TIP]
> This README was generated with trickery
> `trickery generate -i ./prompts/trickery_readme.md > README.md`

## How to install

If you have Rust/Cargo installed, you can install `trickery` with:

```bash
cargo install --git https://github.com/chaliy/trickery.git
trickery --help
```

### ZSH

```bash
trickery completion zsh > $ZSH/cache/completions/_trickery
compinit
trickery <TAB>
```

## Dad Joke

Why did the scarecrow win an award? Because he was outstanding in his field!
