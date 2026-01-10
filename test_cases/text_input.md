# Test: Text Input (Auto-Detection)

## Abstract
Validates that --input auto-detects file paths vs direct text input.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`

## Steps

### 1. Text input (generate)
**Run:** `trickery generate -i "Tell me a short joke"`
**Expect:** LLM response with a joke (input treated as text since no such file exists)

### 2. File input (generate)
**Run:** `trickery generate -i prompts/dad_jokes.md`
**Expect:** LLM response based on file content (file exists, so reads from it)

### 3. Text input (image)
**Run:** `trickery image -i "A simple red circle on white background" -s /tmp/test_circle.png`
**Expect:** Image saved to /tmp/test_circle.png

### 4. Multi-line text input
**Run:**
```bash
trickery generate -i "You are a poet.

Write a haiku about:
- The moon
- Silence"
```
**Expect:** Haiku response

### 5. Text with template variables
**Run:** `trickery generate -i "Hello {{ name }}, you work at {{ company }}." --var name=Alice --var company=Acme`
**Expect:** Response references "Alice" and "Acme"

### 6. Long text input
**Run:**
```bash
trickery generate -i "$(cat <<'EOF'
You are a technical writer. Please summarize the following:

1. Rust is a systems programming language focused on safety.
2. It prevents null pointer exceptions through its ownership system.
3. The borrow checker ensures memory safety at compile time.
4. Cargo is the package manager and build system.
5. Crates are Rust packages published to crates.io.

Provide a 2-sentence summary.
EOF
)"
```
**Expect:** 2-sentence summary of Rust features

### 7. Error: missing --input
**Run:** `trickery generate`
**Expect:** Error about missing --input

### 8. Image with text and auto-generated filename
**Run:** `trickery image -i "Blue square"`
**Expect:** Image saved to image-xxxxx.png (auto-generated filename, not based on input text)

### 9. Text input with JSON output
**Run:** `trickery generate -i "Say hello" -o json`
**Expect:** JSON output with "output" field

### 10. Text input with model selection
**Run:** `trickery generate -i "Count to 5" -m gpt-4o-mini`
**Expect:** Response with numbers 1-5
