use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::aot::{generate, Shell};
use serde::ser;
use std::io;

use commands::{generate::GenerateArgs, image::ImageArgs, CommandExec, CommandExecutionContext};
use output::write_command_stdout_as_json;

mod commands;
mod error;
mod output;
mod provider;
mod trickery;

const LONG_ABOUT: &str = "\
Magic tool to generate things using LLM.

Trickery is a CLI tool for generating textual artifacts using Large Language Models.
It supports Jinja2-like template variables in prompts, model selection, and is
designed for CI/CD integration.

ENVIRONMENT VARIABLES:
  OPENAI_API_KEY    Required. Your OpenAI API key for authentication.

For comprehensive help with all options and examples, use: trickery help --full";

/// Magic tool to generate things
#[derive(Parser)]
#[command(author, version, about, long_about = LONG_ABOUT, disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Type of the output format
    #[arg(short, long, global = true)]
    output: Option<Output>,
}

#[derive(clap::ValueEnum, Clone)]
enum Output {
    Json,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate content
    Generate(GenerateArgs),
    /// Generate or edit images
    Image(ImageArgs),
    /// Outputs the completion file for given shell
    Completion {
        #[arg(index = 1, value_enum)]
        shell: Shell,
    },
    /// Print help information
    Help {
        /// Print comprehensive help with all options and examples
        #[arg(long)]
        full: bool,
    },
}

impl Cli {
    async fn exec_command<T>(&self, executor: &impl CommandExec<T>)
    where
        T: ser::Serialize,
    {
        match executor.exec(self).await {
            Ok(result) => {
                if let Some(Output::Json) = self.output {
                    write_command_stdout_as_json(&*result)
                }
            }
            Err(err) => {
                error::print_error(err.as_ref());
                std::process::exit(1);
            }
        }
    }

    pub fn is_interactive(&self) -> bool {
        self.output.is_none()
    }
}

impl CommandExecutionContext for Cli {
    fn get_cli(&self) -> &Cli {
        self
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate(args)) => {
            cli.exec_command(args).await;
        }
        Some(Commands::Image(args)) => {
            cli.exec_command(args).await;
        }
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            eprintln!("Generating completion file for {shell}...");
            generate(*shell, &mut cmd, name, &mut io::stdout());
        }
        Some(Commands::Help { full }) => {
            if *full {
                print_full_help();
            } else {
                Cli::command().print_help().unwrap();
            }
        }
        None => {}
    }
}

fn print_full_help() {
    print!(
        r#"# trickery - CLI tool for generating textual artifacts using LLM

## Overview

Trickery is a command-line tool for generating content using Large Language Models.
It supports Jinja2-like template variables in prompts, model selection, reasoning
level configuration, and is designed for CI/CD integration.

## Installation

```bash
cargo install trickery
```

## Environment Variables

- `OPENAI_API_KEY` (required): Your OpenAI API key for authentication

## Global Options

- `-o, --output <FORMAT>`: Output format (json). When set, outputs structured JSON
- `-h, --help`: Print help (use `--help` for detailed info)
- `-V, --version`: Print version

## Commands

### generate - Generate content from prompts

Generate text content from a prompt template file.

**Options:**
- `-i, --input <FILE>`: Path to the input prompt file (required)
- `-v, --var <KEY=VALUE>`: Variables to be used in prompt (can be repeated)
- `-m, --model <MODEL>`: Model to use (e.g., gpt-5.2, gpt-5-mini, o1, o3-mini)
- `-r, --reasoning <LEVEL>`: Reasoning level for o1/o3 models: low, medium, high
- `--max-tokens <N>`: Maximum tokens in response
- `--image <PATH|URL>`: Image files or URLs for multimodal prompts (can be repeated)
- `--image-detail <LEVEL>`: Image detail level: auto, low, high (default: auto)

**Examples:**

```bash
# Basic generation from a prompt file
trickery generate -i prompts/greeting.md

# With template variables
trickery generate -i prompts/email.md --var name=John --var topic="Project Update"

# Using a specific model
trickery generate -i prompts/code.md -m gpt-5.2

# With reasoning (for o1/o3 models)
trickery generate -i prompts/analysis.md -m o3-mini -r high

# JSON output for CI/CD
trickery generate -i prompts/data.md -o json

# Multimodal with image input
trickery generate -i prompts/describe.md --image photo.jpg
trickery generate -i prompts/compare.md --image img1.png --image img2.png
```

### image - Generate or edit images

Generate new images or edit existing ones from a prompt template file.

**Options:**
- `-i, --input <FILE>`: Path to the input prompt file (required)
- `-s, --save <FILE>`: Output file path (auto-generated if not provided)
- `-v, --var <KEY=VALUE>`: Variables to be used in prompt (can be repeated)
- `-m, --model <MODEL>`: Model to use (e.g., gpt-4.1, gpt-5, gpt-5.2)
- `--image <PATH|URL>`: Input image files or URLs for editing (can be repeated)
- `--size <SIZE>`: Image size: auto, 1024x1024, 1024x1536 (portrait), 1536x1024 (landscape)
- `--quality <QUALITY>`: Image quality: auto, low, medium, high
- `--format <FORMAT>`: Output format: png, jpeg, webp
- `--background <BG>`: Background: auto, transparent, opaque
- `--action <ACTION>`: Action: auto, generate, edit
- `--compression <0-100>`: Compression level for jpeg/webp formats

**Examples:**

```bash
# Generate an image from a prompt
trickery image -i prompts/logo.md

# Save to specific file
trickery image -i prompts/icon.md -s output/icon.png

# With template variables
trickery image -i prompts/banner.md --var title="Welcome" --var color=blue

# High quality landscape image
trickery image -i prompts/landscape.md --size 1536x1024 --quality high

# Edit an existing image
trickery image -i prompts/edit.md --image original.png --action edit

# Transparent background (for logos/icons)
trickery image -i prompts/logo.md --background transparent --format png

# JSON output for CI/CD
trickery image -i prompts/asset.md -o json
```

### completion - Generate shell completions

Generate shell completion scripts for bash, zsh, fish, elvish, or powershell.

**Usage:**
```bash
trickery completion <SHELL>
```

**Supported shells:** bash, zsh, fish, elvish, powershell

**Examples:**

```bash
# Generate bash completions
trickery completion bash > ~/.local/share/bash-completion/completions/trickery

# Generate zsh completions
trickery completion zsh > ~/.zfunc/_trickery

# Generate fish completions
trickery completion fish > ~/.config/fish/completions/trickery.fish
```

## Template Variables

Prompt files support Jinja2-style template variables using `{{{{ variable }}}}` syntax.

**Example prompt file (prompts/email.md):**
```
Write a professional email to {{{{ name }}}} about {{{{ topic }}}}.
Keep it concise and friendly.
```

**Usage:**
```bash
trickery generate -i prompts/email.md --var name="Alice" --var topic="quarterly review"
```

## Exit Codes

- `0`: Success
- `1`: Error (missing file, API error, invalid arguments, etc.)

## See Also

- Project repository: https://github.com/chaliy/trickery
- OpenAI API documentation: https://platform.openai.com/docs
"#
    );
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
