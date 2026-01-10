# Image Support

Trickery supports multimodal prompts with images, allowing you to send images alongside text prompts to vision-capable LLMs like GPT-5.

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
trickery generate -i prompts/describe_image.md --image test_data/example_images/image2.png
```

Where `prompts/describe_image.md` contains:
```
Describe what you see in this image in detail.
```

### Image from URL

```bash
trickery generate -i prompts/describe_image.md --image https://www.google.com/images/branding/googlelogo/2x/googlelogo_color_272x92dp.png
```

### Multiple Images

```bash
trickery generate -i prompts/catalog_images.md \
  --image test_data/example_images/image1.png \
  --image test_data/example_images/image2.png \
  --image test_data/example_images/image3.jpg
```

### With Detail Level

```bash
# Low detail for quick classification
trickery generate -i prompts/describe_image.md --image test_data/example_images/image1.png --image-detail low
```

### Combined with Variables

```bash
trickery generate -i prompts/review_ui.md \
  --image test_data/example_images/image2.png \
  --var focus="accessibility" \
  --var format="bullet points"
```

Where `prompts/review_ui.md` contains:
```
Review this UI screenshot focusing on {{ focus }}.
Provide feedback in {{ format }}.
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
- `gpt-5-mini` (default)
- `gpt-5.2`

Example with explicit model:
```bash
trickery generate -i prompts/describe_image.md --image test_data/example_images/image1.png --model gpt-5.2
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
