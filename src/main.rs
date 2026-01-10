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

/// Magic tool to generate things
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
        None => {}
    }
}

#[test]
fn verify_cli() {
    Cli::command().debug_assert();
}
