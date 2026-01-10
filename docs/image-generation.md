# Image Generation

Trickery supports generating and editing images using OpenAI's Responses API with the `image_generation` tool. This allows you to create images from text descriptions, edit existing images, and highlight or modify areas in images.

## CLI Arguments

### `--input <PATH>` / `-i <PATH>`

Path to the prompt template file. Supports Jinja2-style `{{ variable }}` substitution.

### `--save <PATH>` / `-s <PATH>` (optional)

Output file path for the generated image. If not provided, a filename is auto-generated from the input file name with a random 5-character suffix (e.g., `diagram-a3f5x.png`).

### `--image <PATH_OR_URL>`

Input image for editing. Can be:
- A local file path (automatically encoded as base64)
- An HTTP/HTTPS URL

This argument can be repeated multiple times to include multiple images (up to 16).

### `--size <SIZE>`

Image dimensions. Options:
- `auto` (default) - Let the model decide
- `1024x1024` or `square` - Square format
- `1024x1536` or `portrait` - Portrait format
- `1536x1024` or `landscape` - Landscape format

### `--quality <LEVEL>`

Image quality. Options:
- `auto` (default) - Let the model decide
- `low` - Faster, lower cost
- `medium` - Balanced
- `high` - Best quality, higher cost

### `--format <FORMAT>`

Output format:
- `png` (default) - Lossless, supports transparency
- `jpeg` - Smaller file size
- `webp` - Modern format, good compression

### `--background <TYPE>`

Background handling:
- `auto` (default) - Let the model decide
- `transparent` - Transparent background (requires PNG or WebP)
- `opaque` - Solid background

### `--action <ACTION>`

Generation behavior:
- `auto` (default) - Model decides whether to generate or edit
- `generate` - Always create new image
- `edit` - Always edit input image

### `--compression <0-100>`

Compression level for JPEG and WebP formats. Default is 100 (highest quality).

### `--model <MODEL>` / `-m <MODEL>`

Model to use. Recommended:
- `gpt-4.1` (default)
- `gpt-5`
- `gpt-5.2`

Note: These models use GPT Image models internally (`gpt-image-1.5`, `gpt-image-1`).

### `--var <KEY=VALUE>` / `-v <KEY=VALUE>`

Template variables for prompt substitution.

## Examples

### Generate from Description

```bash
# Simple generation with explicit filename
trickery image -i prompts/generate_diagram.md --save architecture.png

# Auto-generated filename (e.g., generate_diagram-a3f5x.png)
trickery image -i prompts/generate_diagram.md

# With quality settings
trickery image -i prompts/generate_diagram.md -s architecture.png \
  --size 1536x1024 \
  --quality high
```

See [prompts/generate_diagram.md](../prompts/generate_diagram.md) for the prompt template.

### Edit Existing Image

```bash
# Make an image look realistic
trickery image -i prompts/make_realistic.md \
  --image test_data/example_images/image1.png \
  --save output.png

# Edit with custom instruction
trickery image -i prompts/edit_image.md \
  --image test_data/example_images/image2.png \
  --save modified.png \
  -v instruction="make it green on pink"
```

See [prompts/make_realistic.md](../prompts/make_realistic.md) and [prompts/edit_image.md](../prompts/edit_image.md).

### Highlight Areas in Image

```bash
trickery image -i prompts/highlight_humans.md \
  --image test_data/example_images/image3.jpg \
  --save highlighted.png
```

See [prompts/highlight_humans.md](../prompts/highlight_humans.md) for the prompt template.

### With Template Variables

```bash
trickery image -i prompts/generate_icon.md \
  --save icon.png \
  -v subject="rocket" \
  -v style="flat design"
```

See [prompts/generate_icon.md](../prompts/generate_icon.md) for the prompt template.

### Multiple Input Images

```bash
# Combine elements from multiple images
trickery image -i prompts/edit_image.md \
  --image test_data/example_images/image1.png \
  --image test_data/example_images/image2.png \
  --save composite.png \
  -v instruction="combine these images into one scene"
```

### Transparent Background

```bash
trickery image -i prompts/generate_icon.md \
  --save logo.png \
  --background transparent \
  --format png \
  -v subject="star" \
  -v style="simple outline"
```

### JSON Output

```bash
trickery image -i prompts/generate_diagram.md --save result.png -o json
```

Output:
```json
{
  "output_path": "result.png",
  "revised_prompt": "A clean system architecture diagram..."
}
```

## How It Works

1. **Template Processing**: Variables in `{{ var }}` format are substituted before sending
2. **Input Images**: Local files are encoded as base64 data URLs
3. **API Call**: Uses OpenAI's Responses API with `image_generation` tool
4. **Prompt Optimization**: The model automatically revises your prompt for better results
5. **Output**: Base64 image data is decoded and saved to the output file

## Prompting Tips

- Use action verbs like "draw", "edit", "create", "generate"
- For editing, say "edit the image by..." rather than "merge" or "combine"
- Be specific about style, colors, and composition
- For diagrams, specify the elements and their relationships

## Supported Input Formats

Local files are automatically detected by extension:
- PNG (`.png`)
- JPEG (`.jpg`, `.jpeg`)
- WebP (`.webp`)

Maximum input image size: 50MB per image, up to 16 images.

## Cost Considerations

Image generation costs vary by quality:

| Quality | Approximate Cost |
|---------|------------------|
| `low`   | ~$0.02/image     |
| `medium`| ~$0.07/image     |
| `high`  | ~$0.19/image     |

Larger sizes and multiple input images increase costs.

## Error Handling

Common errors:
- **File not found**: Verify the image path exists
- **Permission denied**: Check file permissions
- **No image generated**: API didn't return an image (check prompt)
- **API error**: Ensure your API key has access to image generation
