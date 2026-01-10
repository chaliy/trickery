# Test: Text Input

## Abstract
Validates direct text input via --text/-t option for generate and image commands.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`

## Steps

### 1. Basic text input (generate)
**Run:** `trickery generate -t "Tell me a short joke"`
**Expect:** LLM response with a joke

### 2. Basic text input (image)
**Run:** `trickery image -t "A simple red circle on white background" -s /tmp/test_circle.png`
**Expect:** Image saved to /tmp/test_circle.png

### 3. Multi-line text input
**Run:**
```bash
trickery generate -t "You are a poet.

Write a haiku about:
- The moon
- Silence"
```
**Expect:** Haiku response

### 4. Text with template variables
**Run:** `trickery generate -t "Hello {{ name }}, you work at {{ company }}." --var name=Alice --var company=Acme`
**Expect:** Response references "Alice" and "Acme"

### 5. Long text input
**Run:**
```bash
trickery generate -t "$(cat <<'EOF'
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

### 6. Error: both --input and --text
**Run:** `trickery generate -i prompts/dad_jokes.md -t "Hello"`
**Expect:** Error: "Cannot specify both --input and --text"

### 7. Error: neither --input nor --text
**Run:** `trickery generate`
**Expect:** Error: "Either --input or --text is required"

### 8. Image with text and auto-generated filename
**Run:** `trickery image -t "Blue square"`
**Expect:** Image saved to image-xxxxx.png (auto-generated filename)

### 9. Text input with JSON output
**Run:** `trickery generate -t "Say hello" -o json`
**Expect:** JSON output with "output" field

### 10. Text input with model selection
**Run:** `trickery generate -t "Count to 5" -m gpt-4o-mini`
**Expect:** Response with numbers 1-5
