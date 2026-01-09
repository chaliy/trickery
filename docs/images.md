# Image Support

Trickery supports multimodal prompts with images, allowing you to send images alongside text prompts to vision-capable LLMs like GPT-4o.

## CLI Arguments

### `--image <PATH_OR_URL>`

Specify an image to include in the prompt. Can be:
- A local file path (automatically encoded as base64)
- An HTTP/HTTPS URL (passed directly to the API)

This argument can be repeated multiple times to include multiple images.

### `--image-detail <LEVEL>`

Control the detail level for image processing. Options:
- `auto` (default) - Let the model decide
- `low` - Lower resolution, faster processing, fewer tokens
- `high` - Higher resolution, more detailed analysis, more tokens

## Supported Formats

Local files are automatically detected by extension:
- PNG (`.png`)
- JPEG (`.jpg`, `.jpeg`)
- GIF (`.gif`)
- WebP (`.webp`)

Unknown extensions default to PNG MIME type.

## Examples

### Single Local Image

```bash
trickery generate -i prompts/describe.md --image screenshot.png
```

Where `prompts/describe.md` contains:
```
Describe what you see in this image in detail.
```

### Image from URL

```bash
trickery generate -i prompts/analyze.md --image https://example.com/chart.png
```

### Multiple Images

```bash
trickery generate -i prompts/compare.md \
  --image before.png \
  --image after.png
```

Where `prompts/compare.md` contains:
```
Compare these two images and describe the differences.
```

### With Detail Level

```bash
# High detail for detailed analysis
trickery generate -i prompts/ocr.md --image document.png --image-detail high

# Low detail for quick classification
trickery generate -i prompts/classify.md --image photo.jpg --image-detail low
```

### Combined with Variables

```bash
trickery generate -i prompts/review.md \
  --image ui-screenshot.png \
  --var focus="accessibility" \
  --var format="bullet points"
```

Where `prompts/review.md` contains:
```
Review this UI screenshot focusing on {{ focus }}.
Provide feedback in {{ format }}.
```

### JSON Output

```bash
trickery generate -i prompts/extract.md \
  --image receipt.jpg \
  --output json
```

## How It Works

1. **Local files** are read and converted to base64 data URLs:
   ```
   data:image/png;base64,iVBORw0KGgo...
   ```

2. **URLs** are passed directly to the API without modification.

3. The message sent to the API includes multiple content parts:
   - Text part with the prompt
   - Image URL parts for each image

## Model Requirements

Image support requires a vision-capable model. Recommended models:
- `gpt-4o` (default if not specified)
- `gpt-4o-mini`
- `gpt-4-turbo`

Example with explicit model:
```bash
trickery generate -i prompt.md --image photo.png --model gpt-4o
```

## Token Considerations

Images consume tokens based on their size and detail level:

| Detail Level | Approximate Tokens |
|--------------|-------------------|
| `low`        | ~85 tokens        |
| `high`       | 85-1700+ tokens   |
| `auto`       | Model decides     |

For cost optimization:
- Use `--image-detail low` for simple classification tasks
- Use `--image-detail high` for OCR, detailed analysis, or small text
- Multiple images multiply token usage

## Error Handling

Common errors:
- **File not found**: Verify the image path exists
- **Permission denied**: Check file permissions
- **Unsupported format**: Use a supported image format
- **API error**: Ensure your model supports vision

## Example Prompt Templates

### `test_cases/image_description.md`
```
Describe the main subject of this image in one sentence.
```

### `test_cases/image_comparison.md`
```
You are shown two images. Compare them and list the key differences.
```
