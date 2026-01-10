# Text Input

## Abstract

Trickery supports direct text input via the `--text`/`-t` option as an alternative to reading prompts from files. This enables quick one-off generations without creating temporary files, supports long multi-line prompts, and integrates with shell scripting workflows.

## Requirements

### Input Options

Both `generate` and `image` commands support two mutually exclusive input methods:

1. **File input** (`-i, --input <FILE>`): Read prompt from a file
2. **Text input** (`-t, --text <TEXT>`): Use text directly as the prompt

### Validation Rules

- Exactly one of `--input` or `--text` must be provided
- If both are provided: error "Cannot specify both --input and --text"
- If neither is provided: error "Either --input or --text is required"

### Text Input Behavior

- Text is used directly as the template content
- Template variable substitution (`{{ var }}`) works with text input
- For `image` command, output filename defaults to `image-xxxxx.png` when no input file

### Long Text Support

The `--text` option supports:

- Multi-line strings (using shell quoting)
- Special characters and Unicode
- Very long prompts (limited only by shell argument length)

### Shell Integration

Examples of passing long text:

```bash
# Multi-line with shell quoting
trickery generate -t "Line 1
Line 2
Line 3"

# Using heredoc
trickery generate -t "$(cat <<'EOF'
You are a helpful assistant.

Please analyze the following:
- Point 1
- Point 2
EOF
)"

# From pipe (when combined with xargs or similar)
echo "Generate a poem" | xargs -I {} trickery generate -t "{}"
```

## Design Choices

### Why not stdin?

Stdin support was considered but deferred because:
1. Adds complexity for detecting interactive vs piped input
2. Conflicts with potential future stdin use for binary data (images)
3. Shell heredocs and `$()` provide adequate workarounds
4. `--text` is explicit and predictable

### Short flag `-t`

The `-t` short flag was chosen because:
- Mnemonic: "t" for "text"
- Commonly used in other CLI tools
- Does not conflict with existing flags
