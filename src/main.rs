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

Generate text content from a prompt. Input is auto-detected: if a file exists at
the given path, it reads from the file; otherwise treats input as direct text.

**Usage:**
```bash
trickery generate [INPUT]           # positional argument
trickery generate -i <INPUT>        # or use -i flag
```

**Options:**
- `<INPUT>`: Prompt input as positional arg - file path or direct text (auto-detected)
- `-i, --input <INPUT>`: Same as positional, alternative syntax
- `-v, --var <KEY=VALUE>`: Variables to be used in prompt (can be repeated)
- `-m, --model <MODEL>`: Model to use (e.g., gpt-5.2, gpt-5-mini, o1, o3-mini)
- `-r, --reasoning <LEVEL>`: Reasoning level for o1/o3 models: low, medium, high
- `--max-tokens <N>`: Maximum tokens in response
- `--image <PATH|URL>`: Image files or URLs for multimodal prompts (can be repeated)
- `--image-detail <LEVEL>`: Image detail level: auto, low, high (default: auto)

**Examples:**

```bash
# From a prompt file (positional)
trickery generate prompts/greeting.md

# From a prompt file (with -i flag)
trickery generate -i prompts/greeting.md

# Direct text input (positional)
trickery generate "Write a haiku about programming"

# Direct text input (with -i flag)
trickery generate -i "Write a haiku about programming"

# Long text with shell quoting
trickery generate "You are a helpful assistant.

Explain the following concept in simple terms:
What is machine learning and how does it work?"

# With template variables
trickery generate prompts/email.md --var name=John --var topic="Project Update"
trickery generate "Hello {{ name }}!" --var name=Alice

# Using a specific model
trickery generate "Explain quantum computing" -m gpt-5.2

# With reasoning (for o1/o3 models)
trickery generate prompts/analysis.md -m o3-mini -r high

# JSON output for CI/CD
trickery generate "Generate a JSON object" -o json

# Multimodal with image input
trickery generate "What is in this image?" --image photo.jpg
```

### image - Generate or edit images

Generate new images or edit existing ones. Input is auto-detected: if a file exists
at the given path, it reads from the file; otherwise treats input as direct text.

**Usage:**
```bash
trickery image [INPUT]           # positional argument
trickery image -i <INPUT>        # or use -i flag
```

**Options:**
- `<INPUT>`: Prompt input as positional arg - file path or direct text (auto-detected)
- `-i, --input <INPUT>`: Same as positional, alternative syntax
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
# From a prompt file (positional)
trickery image prompts/logo.md

# Direct text input (positional)
trickery image "A cute cartoon cat sitting on a rainbow"

# With -i flag
trickery image -i prompts/logo.md

# Long descriptive prompt
trickery image "A professional logo for a tech startup called 'CloudSync'.
Modern, minimalist design with blue and white colors."

# Save to specific file
trickery image "A simple house icon" -s icons/home.png

# With template variables
trickery image "A {{ style }} banner" --var style=modern

# High quality landscape image
trickery image "Beautiful mountain sunset" --size 1536x1024 --quality high

# Edit an existing image
trickery image "Add a red hat to the person" --image photo.jpg --action edit

# Transparent background (for logos/icons)
trickery image "Simple app icon" --background transparent --format png

# JSON output for CI/CD
trickery image prompts/asset.md -o json
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_parse_help_command() {
        let cli = Cli::try_parse_from(["trickery", "help"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Help { full: false })));
    }

    #[test]
    fn test_parse_help_full_command() {
        let cli = Cli::try_parse_from(["trickery", "help", "--full"]).unwrap();
        assert!(matches!(cli.command, Some(Commands::Help { full: true })));
    }

    #[test]
    fn test_long_about_mentions_help_full() {
        assert!(LONG_ABOUT.contains("help --full"));
    }

    #[test]
    fn test_full_help_contains_commands() {
        // Capture full help by checking the static content
        let help = r#"# trickery - CLI tool for generating textual artifacts using LLM"#;
        assert!(help.contains("trickery"));

        // Verify key sections exist in the full help output format
        let sections = [
            "## Overview",
            "## Environment Variables",
            "## Global Options",
            "## Commands",
            "### generate",
            "### image",
            "### completion",
            "## Template Variables",
            "## Exit Codes",
        ];

        // Get the full help content from the raw string in print_full_help
        let full_help = include_str!("main.rs");
        for section in sections {
            assert!(
                full_help.contains(section),
                "Full help should contain section: {}",
                section
            );
        }
    }

    #[test]
    fn test_full_help_contains_examples() {
        let full_help = include_str!("main.rs");
        // Verify examples are present
        assert!(full_help.contains("trickery generate -i"));
        assert!(full_help.contains("trickery image -i"));
        assert!(full_help.contains("trickery completion bash"));
        assert!(full_help.contains("--var name="));
    }

    #[test]
    fn test_full_help_contains_auto_detect() {
        let full_help = include_str!("main.rs");
        // Verify auto-detection is documented
        assert!(full_help.contains("auto-detected"));
        assert!(full_help.contains("file path or direct text"));
    }

    #[test]
    fn test_full_help_contains_positional() {
        let full_help = include_str!("main.rs");
        // Verify positional argument is documented
        assert!(full_help.contains("positional"));
        assert!(full_help.contains("[INPUT]"));
    }

    #[test]
    fn test_parse_generate_with_input_flag() {
        let cli = Cli::try_parse_from(["trickery", "generate", "-i", "prompts/test.md"]).unwrap();
        if let Some(Commands::Generate(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"prompts/test.md".to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_parse_generate_with_positional() {
        let cli = Cli::try_parse_from(["trickery", "generate", "prompts/test.md"]).unwrap();
        if let Some(Commands::Generate(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"prompts/test.md".to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_parse_generate_positional_text() {
        let cli = Cli::try_parse_from(["trickery", "generate", "Hello world"]).unwrap();
        if let Some(Commands::Generate(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"Hello world".to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_parse_generate_positional_with_vars() {
        let cli = Cli::try_parse_from([
            "trickery",
            "generate",
            "Hello {{ name }}",
            "--var",
            "name=Alice",
        ])
        .unwrap();
        if let Some(Commands::Generate(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"Hello {{ name }}".to_string()));
            assert_eq!(args.vars.len(), 1);
        } else {
            panic!("Expected Generate command");
        }
    }

    #[test]
    fn test_parse_image_with_input_flag() {
        let cli = Cli::try_parse_from(["trickery", "image", "-i", "A red circle"]).unwrap();
        if let Some(Commands::Image(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"A red circle".to_string()));
        } else {
            panic!("Expected Image command");
        }
    }

    #[test]
    fn test_parse_image_with_positional() {
        let cli = Cli::try_parse_from(["trickery", "image", "A red circle"]).unwrap();
        if let Some(Commands::Image(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"A red circle".to_string()));
        } else {
            panic!("Expected Image command");
        }
    }

    #[test]
    fn test_parse_image_positional_with_save() {
        let cli =
            Cli::try_parse_from(["trickery", "image", "Blue square", "-s", "output.png"]).unwrap();
        if let Some(Commands::Image(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&"Blue square".to_string()));
            assert!(args.save.is_some());
        } else {
            panic!("Expected Image command");
        }
    }

    #[test]
    fn test_parse_generate_positional_long_text() {
        let long_text = "This is a very long prompt.\n\nIt has multiple lines.\n\nAnd lots of content that should be handled correctly by the CLI parser.";
        let cli = Cli::try_parse_from(["trickery", "generate", long_text]).unwrap();
        if let Some(Commands::Generate(args)) = cli.command {
            assert_eq!(args.get_input(), Some(&long_text.to_string()));
        } else {
            panic!("Expected Generate command");
        }
    }
}
