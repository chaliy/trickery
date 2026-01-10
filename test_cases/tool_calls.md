# Test: Tool Calls (Agentic Generation)

## Abstract
Validates that the tool calling feature works correctly, including the agentic loop with the current_time tool.

## Prerequisites
- `cargo install --path .`
- `OPENAI_API_KEY` environment variable set

## Steps

### 1. Basic tool call with current_time
**Prompt file:** Create `prompts/time-test.md`:
```
What is the current date and time? Use the current_time tool to find out, then tell me the result.
```

**Run:** `trickery generate -i prompts/time-test.md --tool current_time`

**Expect:** Response should include the current date/time, demonstrating that the tool was called and its result was used.

### 2. Tool call with specific format
**Prompt file:** Create `prompts/time-human.md`:
```
Tell me what time it is right now in a human-friendly format. Use the current_time tool with format "human".
```

**Run:** `trickery generate -i prompts/time-human.md --tool current_time`

**Expect:** Response should include a human-readable time like "January 15, 2024 3:30 PM".

### 3. Unknown tool error
**Run:** `trickery generate -i prompts/time-test.md --tool unknown_tool`

**Expect:** Error message indicating unknown tool and listing available tools.

### 4. Multiple tool specification
**Run:** `trickery generate -i prompts/time-test.md --tool current_time --tool current_time`

**Expect:** Should work (duplicate tools are allowed, just redundant).

### 5. Max iterations limit
**Prompt file:** Create `prompts/time-loop.md`:
```
Keep calling the current_time tool and report each result.
```

**Run:** `trickery generate -i prompts/time-loop.md --tool current_time --max-iterations 3`

**Expect:** Should complete within 3 iterations. If the LLM keeps calling tools, it should stop at the limit.

### 6. Tool calls with JSON output
**Run:** `trickery generate -i prompts/time-test.md --tool current_time -o json`

**Expect:** JSON output with the response in the "output" field.

### 7. Image command with tool pre-processing
**Prompt file:** Create `prompts/daily-art.md`:
```
Create an abstract art piece that represents the current time of day. First check what time it is.
```

**Run:** `trickery image -i prompts/daily-art.md --tool current_time`

**Expect:** Image generated with a prompt that was enhanced using the current time.

### 8. No tools - simple generation
**Run:** `trickery generate -i prompts/time-test.md`

**Expect:** Standard generation without tool calls (LLM won't know the actual time).
