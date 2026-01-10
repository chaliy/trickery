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
trickery image -i prompts/diagram.md --save architecture.png

# Auto-generated filename (e.g., diagram-a3f5x.png)
trickery image -i prompts/diagram.md

# With quality settings
trickery image -i prompts/diagram.md -s architecture.png \
  --size 1536x1024 \
  --quality high
```

Where `prompts/diagram.md` contains:
```
Draw a system architecture diagram showing a web app with:
- Frontend (React)
- Backend API (Node.js)
- Database (PostgreSQL)
- Cache layer (Redis)

Use a clean, professional style with labeled boxes and arrows.
```

### Edit Existing Image

```bash
# Make an image look realistic
trickery image -i prompts/make_realistic.md \
  --image input.jpg \
  --save output.png

# Add elements to an image
trickery image -i prompts/add_element.md \
  --image photo.jpg \
  --save modified.png \
  --action edit
```

Where `prompts/make_realistic.md` contains:
```
Edit this image to make it look photorealistic.
Preserve the composition but enhance details and lighting.
```

### Highlight Areas in Image

```bash
trickery image -i prompts/highlight.md \
  --image team_photo.jpg \
  --save highlighted.png
```

Where `prompts/highlight.md` contains:
```
In this image, draw red circles around all people's faces.
Add numbered labels (1, 2, 3...) next to each circle.
```

### With Template Variables

```bash
trickery image -i prompts/generate_icon.md \
  --save icon.png \
  --var subject="rocket" \
  --var style="flat design" \
  --var color="blue"
```

Where `prompts/generate_icon.md` contains:
```
Generate an icon of a {{ subject }}.
Style: {{ style }}
Primary color: {{ color }}
Size: 512x512, centered, with padding.
```

### Multiple Input Images

```bash
# Combine elements from multiple images
trickery image -i prompts/combine.md \
  --image background.jpg \
  --image subject.png \
  --save composite.png
```

### Transparent Background

```bash
trickery image -i prompts/logo.md \
  --save logo.png \
  --background transparent \
  --format png
```

### JSON Output

```bash
trickery image -i prompts/diagram.md --save result.png -o json
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
