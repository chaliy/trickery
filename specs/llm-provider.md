# LLM Provider Abstraction

## Abstract

Trickery uses a minimal, self-contained LLM provider implementation instead of external LLM libraries (like llm-chain). This gives full control over API interactions, reduces dependency bloat, and enables provider-specific optimizations.

The provider abstraction is designed to support multiple backends (OpenAI, Anthropic, Gemini) with a unified interface while allowing provider-specific features like reasoning levels for OpenAI's o1/o3 models.

**Key principle**: Provider handles only the LLM API contract. Template variable substitution happens BEFORE calling the provider (in `trickery/generate.rs`).

## Requirements

### Environment Variables

- `OPENAI_API_KEY` - Required for OpenAI provider
- `OPENAI_BASE_URL` - Optional, defaults to `https://api.openai.com/v1`

### Supported Features

1. **Model Selection** - Users can specify model via `-m/--model` flag
2. **Reasoning Level** - For o1/o3 models, `-r/--reasoning` accepts: low, medium, high
3. **Tool Calls** - Basic function calling support for structured outputs
4. **Max Tokens** - Configurable via `--max-tokens` flag
5. **Content Parts** - Messages use OpenAI's content parts format (text, image_url)

### Default Behavior

- Default model: `gpt-5-mini`
- Temperature is disabled for reasoning models (o1/o3)
- Reasoning effort only sent when model name starts with `o1` or `o3`

### Error Handling

- Missing API key returns clear error message
- HTTP errors include status code and response body
- Invalid responses (no choices) return descriptive error

## Design Choices

### Why not llm-chain?

1. **Dependency weight** - llm-chain pulls in many transitive dependencies
2. **Control** - Direct HTTP calls give full control over request/response handling
3. **Simplicity** - Our use case (single completion calls) doesn't need chain orchestration
4. **Testability** - Easy to mock HTTP responses with mockito

### Separation of Concerns

```
Template Processing (trickery/generate.rs)
├── substitute_variables() - Jinja2-style {{ var }} replacement
└── generate_from_template() - Orchestrates template + provider

LLM Provider (provider/)
├── mod.rs - API contract types (Message, ContentPart, Tool, etc.)
└── openai.rs - OpenAI HTTP client
```

The provider does NOT know about templates - it receives ready-to-send messages.

### Provider Abstraction Structure

```
src/provider/
├── mod.rs      # Common types: Message, ContentPart, Tool, CompletionRequest/Response
└── openai.rs   # OpenAI-specific implementation
```

Future providers (Anthropic, Gemini) will:
- Add new files: `anthropic.rs`, `gemini.rs`
- Implement the same `complete()` pattern
- Use provider-specific env vars (e.g., `ANTHROPIC_API_KEY`)

### Message Format (OpenAI Content Parts)

Messages use the standard OpenAI API format with content as array of parts:

```rust
pub enum ContentPart {
    Text { text: String },
    ImageUrl { image_url: ImageUrl },  // For vision models
}

pub struct Message {
    role: Role,
    content: Option<Vec<ContentPart>>,  // Array of content parts
    tool_calls: Option<Vec<ToolCall>>,
    tool_call_id: Option<String>,
}
```

Serializes to:
```json
{
  "role": "user",
  "content": [
    {"type": "text", "text": "Hello world"}
  ]
}
```

### Request Builder Pattern

```rust
CompletionRequest::new(messages)
    .with_model("gpt-5-mini")
    .with_reasoning_level(ReasoningLevel::High)
    .with_tools(tools)
    .with_max_tokens(1000)
```
