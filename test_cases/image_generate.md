# Test: Image Generation

## Abstract
Validates the `image` command for generating and editing images using OpenAI's Responses API.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`
- Test image available for edit tests (e.g., `test.png`)

## Steps

### 1. Generate image from prompt
**Run:** `trickery image -i prompts/generate_diagram.md --out /tmp/diagram.png`
**Expect:** Image saved to `/tmp/diagram.png`, success message printed

### 2. Generate with size option
**Run:** `trickery image -i prompts/generate_diagram.md --out /tmp/diagram.png --size 1536x1024`
**Expect:** Landscape image (1536x1024) saved to output path

### 3. Generate with quality option
**Run:** `trickery image -i prompts/generate_diagram.md --out /tmp/diagram.png --quality high`
**Expect:** High quality image generated (higher cost, better detail)

### 4. Generate with template variables
**Run:** `trickery image -i prompts/generate_icon.md --out /tmp/icon.png -v subject=rocket -v style="flat design"`
**Expect:** Icon generated based on substituted prompt variables

### 5. Edit existing image
**Run:** `trickery image -i prompts/edit_image.md --image ./test.png --out /tmp/edited.png -v instruction="make it black and white"`
**Expect:** Modified image saved with the requested edit applied

### 6. Transparent background
**Run:** `trickery image -i prompts/generate_icon.md --out /tmp/logo.png --background transparent --format png -v subject=star -v style=simple`
**Expect:** PNG image with transparent background

### 7. JSON output format
**Run:** `trickery image -i prompts/generate_diagram.md --out /tmp/test.png -o json`
**Expect:** JSON output with `output_path` and `revised_prompt` fields

### 8. Multiple input images
**Run:** `trickery image -i prompts/edit_image.md --image ./bg.png --image ./fg.png --out /tmp/composite.png -v instruction="combine these images"`
**Expect:** Image generated using both input images as context
