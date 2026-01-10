# Test: Error Handling

## Abstract
Validates proper error messages for invalid inputs and missing requirements.

## Prerequisites
- `cargo install --path .`

## Steps

### 1. Missing input
**Run:** `trickery generate`
**Expect:** Error: "Input is required"

### 2. Missing API key
**Run:** `unset OPENAI_API_KEY && trickery generate prompts/dad_jokes.md`
**Expect:** Error indicating missing or invalid API key

### 3. Invalid variable format
**Run:** `trickery generate prompts/dad_jokes.md --var invalidformat`
**Expect:** Error: "invalid KEY=VALUE: no `=` found"

### 4. Invalid reasoning level
**Run:** `trickery generate prompts/dad_jokes.md -r invalid`
**Expect:** Error about invalid reasoning level value
