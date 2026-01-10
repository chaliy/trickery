# Text Input

## Abstract

Trickery's `--input` option supports both file paths and direct text, with auto-detection. If the provided value exists as a file, it reads from the file; otherwise, it treats the value as direct prompt text. This enables quick one-off generations without creating temporary files.

## Requirements

### Input Auto-Detection

The `-i, --input` option uses this logic:
1. Check if the input value exists as a file on disk
2. If file exists: read content from the file
3. If file doesn't exist: use the input value directly as prompt text

### Behavior

- File input: `trickery generate -i prompts/greeting.md` reads from file
- Text input: `trickery generate -i "Write a haiku"` uses text directly
- Template variables work with both: `--var name=Alice`
- For `image` command, output filename defaults to `image-xxxxx.png` when input is text

### Long Text Support

The `--input` option supports:

- Multi-line strings (using shell quoting)
- Special characters and Unicode
- Very long prompts (limited only by shell argument length)

### Shell Integration

Examples of passing long text:

```bash
# Multi-line with shell quoting
trickery generate -i "Line 1
Line 2
Line 3"

# Using heredoc
trickery generate -i "$(cat <<'EOF'
You are a helpful assistant.

Please analyze the following:
- Point 1
- Point 2
EOF
)"

# From pipe (when combined with xargs or similar)
echo "Generate a poem" | xargs -I {} trickery generate -i "{}"
```

## Design Choices

### Why auto-detect instead of separate options?

1. Simpler API: one option instead of two
2. Intuitive behavior: file paths look like file paths, text looks like text
3. No ambiguity in practice: prompts rarely look like existing file paths
4. Matches common patterns in other CLI tools (e.g., `curl -d`)

### Edge cases

- If you have a file named "Hello world" and want to use it: the file will be read
- If you want to use text that matches an existing filename: rename the file or use a path like `./filename`
