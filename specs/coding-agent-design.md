# Coding Agent Design

## Abstract

Trickery is designed to be friendly for AI coding agents (Claude, GPT, Copilot, etc.) that need to generate text and images as part of automated workflows. The tool prioritizes discoverability, predictable behavior, and actionable error messages that enable agents to self-correct.

## Requirements

### Discoverability

1. **Full help system** - `trickery help --full` outputs comprehensive documentation with examples, similar to llms.txt format
2. **Command help** - Each command supports `--help` with usage patterns
3. **Shell completions** - `trickery completion <shell>` for bash, zsh, fish, elvish, powershell

### Error Recovery

Errors must include enough context for an agent to understand what went wrong and how to fix it.

1. **Missing API key** - Shows which env var to set and where to get a key
2. **Network errors** - Distinguishes connection vs timeout, suggests retry
3. **API errors** - Includes status code and hints for common codes (401, 429, 500)
4. **File errors** - Explains permission issues, suggests path corrections
5. **Exit codes** - Non-zero exit for errors, zero for success

Example error output:
```
🔑 Missing API Key: OPENAI_API_KEY

ℹ To fix this, set the environment variable:

   export OPENAI_API_KEY=your_api_key_here

ℹ You can get an API key from: https://platform.openai.com/api-keys
```

### Predictable Behavior

1. **Structured output** - `--json` flag outputs machine-readable JSON
2. **Auto-detection** - Input can be file path or direct text, determined automatically
3. **Template variables** - `{{ var }}` syntax with `-v key=value` for reproducible prompts
4. **Consistent flags** - Same flags work across commands where applicable

### CI/CD Integration

1. **No interactive prompts** - All input via arguments or stdin
2. **Environment variables** - API keys and base URL via env vars
3. **Exit codes** - Proper exit codes for scripting
4. **Quiet mode** - Minimal output for piping (`generate` outputs only LLM response)

## Design Choices

### Why detailed error messages?

Agents parse error output to decide next steps. Cryptic errors like "request failed" force guessing. Rich errors with hints ("API key invalid, get one at...") let agents suggest fixes or retry with corrections.

### Why auto-detect file vs text input?

Agents often construct prompts dynamically. Requiring explicit `--file` vs `--text` flags adds friction. Auto-detection (check if path exists) handles both cases naturally:

```bash
# Agent can do either:
trickery generate prompts/template.md
trickery generate "Write a haiku about $TOPIC"
```

### Why template variables instead of string interpolation?

Shell interpolation happens before trickery sees the input, making debugging hard. Template variables (`{{ var }}`) are processed by trickery, visible in the template file, and explicit via `-v` flags.

### Why JSON output option?

Agents parsing natural text is error-prone. `--json` provides structured output:

```json
{
  "content": "Generated text here...",
  "model": "gpt-5-mini",
  "usage": {"prompt_tokens": 10, "completion_tokens": 50}
}
```

## Implementation Notes

- Error formatting: `src/error.rs` - icons, hints, and structured messages
- Help system: `src/main.rs` `print_full_help()` - comprehensive examples
- Auto-detection: `src/commands/generate.rs` - file existence check
- JSON output: `src/output.rs` - serde serialization
