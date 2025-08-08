use clap::{Args, ValueHint};
use serde_json::Value;
use std::path::PathBuf;

fn parse_key_val(s: &str) -> Result<(String, Value), String> {
    let pos = s
        .find('=')
        .ok_or_else(|| format!("invalid KEY=VALUE: no `=` found in `{}`", s))?;
    Ok((
        s[..pos].to_string(),
        Value::String(s[pos + 1..].to_string()),
    ))
}

#[derive(Args)]
pub struct AgentArgs {
    /// Path to the input prompt file
    #[arg(short, long, value_hint = ValueHint::FilePath)]
    pub input: Option<PathBuf>,
    /// Variables to be used in prompt
