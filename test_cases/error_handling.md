# Test: Error Handling

## Abstract
Validates proper error messages for invalid inputs and missing requirements.

## Prerequisites
- `cargo install --path .`

## Steps

### 1. Missing input file
**Run:** `trickery generate`
**Expect:** Error: "Input file path is required"

### 2. Non-existent file
**Run:** `trickery generate -i nonexistent.md`
**Expect:** Error: "Failed to read input file 'nonexistent.md': No such file"

### 3. Missing API key
**Run:** `unset OPENAI_API_KEY && trickery generate -i prompts/dad_jokes.md`
**Expect:** Error indicating missing or invalid API key

### 4. Invalid variable format
**Run:** `trickery generate -i prompts/dad_jokes.md --var invalidformat`
**Expect:** Error: "invalid KEY=VALUE: no `=` found"

### 5. Invalid reasoning level
**Run:** `trickery generate -i prompts/dad_jokes.md -r invalid`
**Expect:** Error about invalid reasoning level value
