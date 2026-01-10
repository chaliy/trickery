# Test: Text Input (Positional and -i Flag)

## Abstract
Validates that input works as positional argument and with -i flag, with auto-detection of file vs text.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`

## Steps

### 1. Positional text input (generate)
**Run:** `trickery generate "Tell me a short joke"`
**Expect:** LLM response with a joke

### 2. Flag text input (generate)
**Run:** `trickery generate -i "Tell me a short joke"`
**Expect:** Same result as positional

### 3. Positional file input (generate)
**Run:** `trickery generate prompts/dad_jokes.md`
**Expect:** LLM response based on file content

### 4. Flag file input (generate)
**Run:** `trickery generate -i prompts/dad_jokes.md`
**Expect:** Same result as positional file input

### 5. Positional text input (image)
**Run:** `trickery image "A red circle" -s /tmp/test_circle.png`
**Expect:** Image saved to /tmp/test_circle.png

### 6. Flag text input (image)
**Run:** `trickery image -i "A red circle" -s /tmp/test_circle2.png`
**Expect:** Image saved to /tmp/test_circle2.png

### 7. Multi-line text input
**Run:**
```bash
trickery generate "You are a poet.

Write a haiku about:
- The moon
- Silence"
```
**Expect:** Haiku response

### 8. Positional with template variables
**Run:** `trickery generate "Hello {{ name }}!" --var name=Alice`
**Expect:** Response references "Alice"

### 9. Long text input
**Run:**
```bash
trickery generate "$(cat <<'EOF'
You are a technical writer. Please summarize:

1. Rust is a systems programming language.
2. It prevents null pointer exceptions.
3. Cargo is the package manager.

Provide a 2-sentence summary.
EOF
)"
```
**Expect:** 2-sentence summary

### 10. Error: missing input
**Run:** `trickery generate`
**Expect:** Error about missing input

### 11. Positional with model selection
**Run:** `trickery generate "Count to 5" -m gpt-4o-mini`
**Expect:** Response with numbers 1-5

### 12. Positional with JSON output
**Run:** `trickery generate "Say hello" -o json`
**Expect:** JSON output with "output" field
