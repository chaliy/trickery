# Test: Error Handling

## Abstract
Validates proper error messages for invalid inputs and missing requirements.

## Prerequisites
- `cargo build` completed successfully

## Steps

### 1. Missing input file
**Run:** `cargo run -- generate`
**Expect:** Error: "Input file path is required"

### 2. Non-existent file
**Run:** `cargo run -- generate -i nonexistent.md`
**Expect:** Error: "Failed to read input file 'nonexistent.md': No such file"

### 3. Missing API key
**Run:** `unset OPENAI_API_KEY && cargo run -- generate -i prompts/dad_jokes.md`
**Expect:** Error indicating missing or invalid API key

### 4. Invalid variable format
**Run:** `cargo run -- generate -i prompts/dad_jokes.md --var invalidformat`
**Expect:** Error: "invalid KEY=VALUE: no `=` found"

### 5. Invalid reasoning level
**Run:** `cargo run -- generate -i prompts/dad_jokes.md -r invalid`
**Expect:** Error about invalid reasoning level value
