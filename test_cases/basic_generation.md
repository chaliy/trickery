# Test: Basic Generation

## Abstract
Validates that trickery can generate output from a simple prompt file without variables.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`

## Steps

### 1. Generate from simple prompt
**Run:** `trickery generate -i prompts/dad_jokes.md`
**Expect:** LLM response printed to stdout (a dad joke)

### 2. Generate with model selection
**Run:** `trickery generate -i prompts/dad_jokes.md -m gpt-4o-mini`
**Expect:** LLM response from specified model

### 3. Generate with max tokens limit
**Run:** `trickery generate -i prompts/dad_jokes.md --max-tokens 50`
**Expect:** Response truncated to approximately 50 tokens
