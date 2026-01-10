# Test: Image Generation

## Abstract
Validates the `image` command for generating and editing images using OpenAI's Responses API.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- `cargo install --path .`
- Test images in `test_data/example_images/` (image1.png, image2.png, image3.jpg)

## Steps

### 1. Generate image from prompt
**Run:** `trickery image -i prompts/generate_diagram.md --save /tmp/diagram.png`
**Expect:** Image saved to `/tmp/diagram.png`, success message printed

### 2. Generate with size option
**Run:** `trickery image -i prompts/generate_diagram.md --save /tmp/diagram.png --size 1536x1024`
**Expect:** Landscape image (1536x1024) saved to output path

### 3. Generate with quality option
**Run:** `trickery image -i prompts/generate_diagram.md --save /tmp/diagram.png --quality high`
**Expect:** High quality image generated (higher cost, better detail)

### 4. Generate with template variables
**Run:** `trickery image -i prompts/generate_icon.md --save /tmp/icon.png -v subject=rocket -v style="flat design"`
**Expect:** Icon generated based on substituted prompt variables

### 5. Edit existing image
**Run:** `trickery image -i prompts/edit_image.md --image test_data/example_images/image1.png --save /tmp/edited.png -v instruction="make it green on pink"`
**Expect:** Modified image saved with the requested edit applied

### 6. Transparent background
**Run:** `trickery image -i prompts/generate_icon.md --save /tmp/logo.png --background transparent --format png -v subject=star -v style=simple`
**Expect:** PNG image with transparent background

### 7. JSON output format
**Run:** `trickery image -i prompts/generate_diagram.md --save /tmp/test.png -o json`
**Expect:** JSON output with `output_path` and `revised_prompt` fields

### 8. Multiple input images
**Run:** `trickery image -i prompts/edit_image.md --image test_data/example_images/image1.png --image test_data/example_images/image2.png --save /tmp/composite.png -v instruction="combine these images"`
**Expect:** Image generated using both input images as context

### 9. Auto-generated filename
**Run:** `trickery image -i prompts/generate_diagram.md`
**Expect:** Image saved to auto-generated filename (e.g., `generate_diagram-a3f5x.png` in current directory), success message shows path

### 10. Short flag for save
**Run:** `trickery image -i prompts/generate_diagram.md -s /tmp/short.png`
**Expect:** Image saved to `/tmp/short.png` using short `-s` flag

### 11. Highlight humans in image
**Run:** `trickery image -i prompts/highlight_humans.md --image test_data/example_images/image3.jpg --save /tmp/highlighted.png`
**Expect:** Image with red circles around humans and numbered labels
