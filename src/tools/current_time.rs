// current_time tool - returns current date/time in various formats.
// This is a simple demonstration tool for the agentic loop.

use super::{function_tool, ToolError, ToolExecutor};
use crate::provider::Tool;
use chrono::{Local, Utc};
use serde::Deserialize;

/// Tool that returns the current date and time
pub struct CurrentTimeTool;

#[derive(Deserialize, Default)]
struct CurrentTimeArgs {
    /// Timezone: "utc" or "local" (default: "local")
    #[serde(default)]
    timezone: Option<String>,
    /// Output format: "iso8601", "rfc2822", "unix", "human" (default: "iso8601")
    #[serde(default)]
    format: Option<String>,
}

impl ToolExecutor for CurrentTimeTool {
    fn name(&self) -> &'static str {
        "current_time"
    }

    fn definition(&self) -> Tool {
        function_tool(
            "current_time",
            "Get the current date and time. Returns the current timestamp in the specified format and timezone.",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "timezone": {
                        "type": "string",
                        "description": "Timezone to use: 'utc' for UTC or 'local' for system local time",
                        "enum": ["utc", "local"],
                        "default": "local"
                    },
                    "format": {
                        "type": "string",
                        "description": "Output format: 'iso8601' (2024-01-15T10:30:00), 'rfc2822' (Mon, 15 Jan 2024 10:30:00 +0000), 'unix' (timestamp in seconds), 'human' (January 15, 2024 10:30 AM)",
                        "enum": ["iso8601", "rfc2822", "unix", "human"],
                        "default": "iso8601"
                    }
                },
                "additionalProperties": false
            }),
        )
    }

    fn execute(&self, arguments: &str) -> Result<String, ToolError> {
        let args: CurrentTimeArgs = if arguments.trim().is_empty() || arguments == "{}" {
            CurrentTimeArgs::default()
        } else {
            serde_json::from_str(arguments)
                .map_err(|e| ToolError::InvalidArguments(e.to_string()))?
        };

        let timezone = args.timezone.as_deref().unwrap_or("local");
        let format = args.format.as_deref().unwrap_or("iso8601");

        let result = match timezone {
            "utc" => format_time_utc(format),
            _ => format_time_local(format),
        };

        Ok(result)
    }
}

fn format_time_utc(format: &str) -> String {
    let now = Utc::now();
    match format {
        "rfc2822" => now.to_rfc2822(),
        "unix" => now.timestamp().to_string(),
        "human" => now.format("%B %d, %Y %I:%M %p UTC").to_string(),
        _ => now.to_rfc3339(), // Default to iso8601
    }
}

fn format_time_local(format: &str) -> String {
    let now = Local::now();
    match format {
        "rfc2822" => now.to_rfc2822(),
        "unix" => now.timestamp().to_string(),
        "human" => now.format("%B %d, %Y %I:%M %p").to_string(),
        _ => now.to_rfc3339(), // Default to iso8601
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_name() {
        let tool = CurrentTimeTool;
        assert_eq!(tool.name(), "current_time");
    }

    #[test]
    fn test_tool_definition() {
        let tool = CurrentTimeTool;
        let def = tool.definition();
        assert_eq!(def.tool_type, "function");
        assert_eq!(def.function.name, "current_time");
        assert!(def.function.description.contains("current date and time"));
    }

    #[test]
    fn test_execute_default() {
        let tool = CurrentTimeTool;
        let result = tool.execute("{}").unwrap();
        // Should return ISO8601 format
        assert!(result.contains("T"));
        assert!(result.contains(":"));
    }

    #[test]
    fn test_execute_empty_args() {
        let tool = CurrentTimeTool;
        let result = tool.execute("").unwrap();
        assert!(result.contains("T"));
    }

    #[test]
    fn test_execute_utc() {
        let tool = CurrentTimeTool;
        let result = tool.execute(r#"{"timezone": "utc"}"#).unwrap();
        assert!(result.contains("+00:00") || result.contains("Z"));
    }

    #[test]
    fn test_execute_unix_format() {
        let tool = CurrentTimeTool;
        let result = tool.execute(r#"{"format": "unix"}"#).unwrap();
        // Unix timestamp should be a number
        assert!(result.parse::<i64>().is_ok());
    }

    #[test]
    fn test_execute_rfc2822_format() {
        let tool = CurrentTimeTool;
        let result = tool.execute(r#"{"format": "rfc2822"}"#).unwrap();
        // RFC2822 format includes day name
        assert!(
            result.contains("Mon")
                || result.contains("Tue")
                || result.contains("Wed")
                || result.contains("Thu")
                || result.contains("Fri")
                || result.contains("Sat")
                || result.contains("Sun")
        );
    }

    #[test]
    fn test_execute_human_format() {
        let tool = CurrentTimeTool;
        let result = tool.execute(r#"{"format": "human"}"#).unwrap();
        // Human format includes month name and AM/PM
        assert!(result.contains("AM") || result.contains("PM"));
    }

    #[test]
    fn test_execute_human_utc() {
        let tool = CurrentTimeTool;
        let result = tool
            .execute(r#"{"timezone": "utc", "format": "human"}"#)
            .unwrap();
        assert!(result.contains("UTC"));
    }

    #[test]
    fn test_execute_invalid_json() {
        let tool = CurrentTimeTool;
        let result = tool.execute("not json");
        assert!(matches!(result, Err(ToolError::InvalidArguments(_))));
    }
}
