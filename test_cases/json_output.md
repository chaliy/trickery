# Test: JSON Output

## Abstract
Validates the `-o json` flag produces structured JSON output.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`
- `jq` installed for JSON validation (optional)

## Steps

### 1. JSON output format
**Run:** `trickery -o json generate prompts/dad_jokes.md`
**Expect:** Output is valid JSON with structure: `{"output": "<response>"}`

### 2. JSON output piped to jq
**Run:** `trickery -o json generate prompts/dad_jokes.md | jq .output`
**Expect:** Extracted output string without JSON wrapper

### 3. Compare interactive vs JSON mode
**Run:** `trickery generate prompts/dad_jokes.md` vs `trickery -o json generate prompts/dad_jokes.md`
**Expect:** Interactive mode prints raw text; JSON mode wraps in object
