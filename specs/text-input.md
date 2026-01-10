# Text Input

## Abstract

Trickery's input supports both file paths and direct text, with auto-detection. If the provided value exists as a file, it reads from the file; otherwise, it treats the value as direct prompt text. Input can be provided as a positional argument or with the `-i` flag.

## Requirements

### Input Methods

Two equivalent ways to provide input:
1. **Positional argument**: `trickery generate "prompt text"`
2. **Named option**: `trickery generate -i "prompt text"`

Both work identically. Positional is preferred for brevity.

### Input Auto-Detection

Once input is provided (either way), this logic applies:
1. Check if the input value exists as a file on disk
2. If file exists: read content from the file
3. If file doesn't exist: use the input value directly as prompt text

### Behavior

```bash
# File input (file exists, content read from file)
trickery generate prompts/greeting.md
trickery generate -i prompts/greeting.md

# Text input (not a file, used as direct prompt)
trickery generate "Write a haiku"
trickery generate -i "Write a haiku"
```

- Template variables work with both: `--var name=Alice`
- For `image` command, output filename defaults to `image-xxxxx.png` when input is text

### Long Text Support

Both positional and `-i` support:

- Multi-line strings (using shell quoting)
- Special characters and Unicode
- Very long prompts (limited only by shell argument length)

### Shell Integration

Examples of passing long text:

```bash
# Multi-line with shell quoting
trickery generate "Line 1
Line 2
Line 3"

# Using heredoc
trickery generate "$(cat <<'EOF'
You are a helpful assistant.

Please analyze the following:
- Point 1
- Point 2
EOF
)"
```

## Design Choices

### Why support both positional and -i?

1. Positional is more natural for quick one-liners
2. `-i` flag maintains backwards compatibility
3. `-i` is clearer when input looks like a flag (edge case)

### Why auto-detect instead of separate options?

1. Simpler API: one input concept
2. Intuitive behavior: file paths look like file paths, text looks like text
3. No ambiguity in practice: prompts rarely look like existing file paths
