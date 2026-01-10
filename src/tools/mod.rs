// Tools module - extensible tool system for agentic workflows.
// Design: Each tool implements the ToolExecutor trait and registers with the registry.
// Tools are discovered by name and executed with JSON arguments.

pub mod current_time;

use crate::provider::{FunctionDef, Tool};
use serde_json::Value;
use std::collections::HashMap;

/// Trait for tool execution. Each tool must implement this.
pub trait ToolExecutor: Send + Sync {
    /// Tool name (must match the function name in tool definition)
    fn name(&self) -> &'static str;

    /// Tool definition for LLM
    fn definition(&self) -> Tool;

    /// Execute the tool with given arguments
    fn execute(&self, arguments: &str) -> Result<String, ToolError>;
}

/// Tool execution error
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)] // Variants are part of public API for tool implementors
pub enum ToolError {
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Tool not found: {0}")]
    NotFound(String),
}

/// Registry of available tools
pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn ToolExecutor>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    /// Create empty registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Create registry with all built-in tools
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        registry.register(Box::new(current_time::CurrentTimeTool));
        registry
    }

    /// Register a tool
    pub fn register(&mut self, tool: Box<dyn ToolExecutor>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Get tool by name
    #[allow(dead_code)] // Part of public API for tool lookup
    pub fn get(&self, name: &str) -> Option<&dyn ToolExecutor> {
        self.tools.get(name).map(|t| t.as_ref())
    }

    /// Get all tool definitions
    pub fn definitions(&self) -> Vec<Tool> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Get tool definitions for specific tool names
    pub fn definitions_for(&self, names: &[String]) -> Vec<Tool> {
        names
            .iter()
            .filter_map(|name| self.tools.get(name).map(|t| t.definition()))
            .collect()
    }

    /// Execute a tool by name
    pub fn execute(&self, name: &str, arguments: &str) -> Result<String, ToolError> {
        match self.tools.get(name) {
            Some(tool) => tool.execute(arguments),
            None => Err(ToolError::NotFound(name.to_string())),
        }
    }

    /// List available tool names
    pub fn available_tools(&self) -> Vec<&str> {
        self.tools.keys().map(|s| s.as_str()).collect()
    }
}

/// Helper to create a function tool definition
pub fn function_tool(name: &str, description: &str, parameters: Value) -> Tool {
    Tool {
        tool_type: "function".to_string(),
        function: FunctionDef {
            name: name.to_string(),
            description: description.to_string(),
            parameters,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_new() {
        let registry = ToolRegistry::new();
        assert!(registry.available_tools().is_empty());
    }

    #[test]
    fn test_registry_with_builtins() {
        let registry = ToolRegistry::with_builtins();
        assert!(registry.get("current_time").is_some());
    }

    #[test]
    fn test_registry_definitions() {
        let registry = ToolRegistry::with_builtins();
        let defs = registry.definitions();
        assert!(!defs.is_empty());
        assert!(defs.iter().any(|d| d.function.name == "current_time"));
    }

    #[test]
    fn test_registry_definitions_for() {
        let registry = ToolRegistry::with_builtins();
        let defs = registry.definitions_for(&["current_time".to_string()]);
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].function.name, "current_time");
    }

    #[test]
    fn test_registry_definitions_for_unknown() {
        let registry = ToolRegistry::with_builtins();
        let defs = registry.definitions_for(&["unknown_tool".to_string()]);
        assert!(defs.is_empty());
    }

    #[test]
    fn test_execute_not_found() {
        let registry = ToolRegistry::new();
        let result = registry.execute("unknown", "{}");
        assert!(matches!(result, Err(ToolError::NotFound(_))));
    }

    #[test]
    fn test_function_tool_helper() {
        let tool = function_tool("test", "Test tool", serde_json::json!({"type": "object"}));
        assert_eq!(tool.tool_type, "function");
        assert_eq!(tool.function.name, "test");
        assert_eq!(tool.function.description, "Test tool");
    }
}
