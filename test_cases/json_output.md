# Test: JSON Output

## Abstract
Validates the `-o json` flag produces structured JSON output.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `jq` installed for JSON validation (optional)

## Steps

### 1. JSON output format
**Run:** `cargo run -- -o json generate -i prompts/dad_jokes.md`
**Expect:** Output is valid JSON with structure: `{"output": "<response>"}`

### 2. JSON output piped to jq
**Run:** `cargo run -- -o json generate -i prompts/dad_jokes.md | jq .output`
**Expect:** Extracted output string without JSON wrapper

### 3. Compare interactive vs JSON mode
**Run:** `cargo run -- generate -i prompts/dad_jokes.md` vs `cargo run -- -o json generate -i prompts/dad_jokes.md`
**Expect:** Interactive mode prints raw text; JSON mode wraps in object
