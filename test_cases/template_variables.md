# Test: Template Variables

## Abstract
Validates Jinja2-style variable substitution in prompt templates.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- Create test prompt: `echo "Hello {{ name }}, you are a {{ role }}." > /tmp/test_vars.md`

## Steps

### 1. Single variable substitution
**Run:** `cargo run -- generate -i /tmp/test_vars.md --var name=Alice`
**Expect:** Prompt renders with "Alice" replacing `{{ name }}`; `{{ role }}` may appear literally or cause template warning

### 2. Multiple variables
**Run:** `cargo run -- generate -i /tmp/test_vars.md --var name=Bob --var role=developer`
**Expect:** Both variables substituted; response references "Bob" and "developer"

### 3. Variable with special characters
**Run:** `cargo run -- generate -i /tmp/test_vars.md --var name="John Doe" --var role="senior engineer"`
**Expect:** Values with spaces handled correctly
