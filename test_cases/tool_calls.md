# Test: Tool Calls (Agentic Generation)

## Abstract
Validates that the tool calling feature works correctly, including the agentic loop with the current_time tool.

## Prerequisites
- `cargo install --path .`
- `OPENAI_API_KEY` environment variable set

## Steps

### 1. Basic tool call with current_time (inline prompt)
**Run:**
```bash
trickery generate "Tell a dad joke about the current time" --tool current_time
```

**Expect:** Response should include a time-related dad joke, demonstrating that the tool was called and its result was used.

### 2. Tool call with specific format request
**Run:**
```bash
trickery generate "What time is it right now? Use human format." --tool current_time
```

**Expect:** Response should include a human-readable time like "January 15, 2024 3:30 PM".

### 3. Unknown tool error
**Run:**
```bash
trickery generate "Hello" --tool unknown_tool
```

**Expect:** Error message indicating unknown tool and listing available tools:
```
Unknown tool 'unknown_tool'. Available tools: current_time
```

### 4. Multiple tool specification
**Run:**
```bash
trickery generate "What time is it?" --tool current_time --tool current_time
```

**Expect:** Should work (duplicate tools are allowed, just redundant).

### 5. Max iterations limit
**Run:**
```bash
trickery generate "Keep calling current_time and report each result" --tool current_time --max-iterations 3
```

**Expect:** Should complete within 3 iterations. If the LLM keeps calling tools, it should stop at the limit.

### 6. Tool calls with JSON output
**Run:**
```bash
trickery generate "What is the current UTC time?" --tool current_time -o json
```

**Expect:** JSON output with the response in the "output" field:
```json
{
  "output": "The current UTC time is..."
}
```

### 7. File-based prompt with tools
**Prompt file:** Create `prompts/time-test.md`:
```
What is the current date and time? Use the current_time tool to find out.
```

**Run:**
```bash
trickery generate -i prompts/time-test.md --tool current_time
```

**Expect:** Response includes the current date/time.

### 8. Image command with tool pre-processing
**Prompt file:** Create `prompts/daily-art.md`:
```
Create abstract art representing the current time of day
```

**Run:**
```bash
trickery image -i prompts/daily-art.md --tool current_time
```

**Expect:** Image generated with a prompt enhanced using the current time.

### 9. No tools - simple generation
**Run:**
```bash
trickery generate "What time is it?"
```

**Expect:** Standard generation without tool calls (LLM won't know the actual current time).

### 10. UTC timezone request
**Run:**
```bash
trickery generate "What is the current time in UTC?" --tool current_time
```

**Expect:** Response includes UTC time, showing the LLM correctly requested UTC timezone from the tool.
