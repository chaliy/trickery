# Test: Image Multimodal

## Abstract
Validates multimodal image input for vision-capable prompts.

## Prerequisites
- `OPENAI_API_KEY` environment variable set
- Test image available (e.g., `test.png` or URL)
- Vision-capable model (gpt-4o, gpt-4o-mini)

## Steps

### 1. Image from local file
**Run:** `cargo run -- generate -i prompts/describe_image.md --image ./test.png`
**Expect:** LLM describes contents of the image

### 2. Image from URL
**Run:** `cargo run -- generate -i prompts/describe_image.md --image "https://example.com/image.jpg"`
**Expect:** LLM fetches and describes the remote image

### 3. Image detail level
**Run:** `cargo run -- generate -i prompts/describe_image.md --image ./test.png --image-detail high`
**Expect:** Higher detail analysis; may take longer and use more tokens

### 4. Multiple images
**Run:** `cargo run -- generate -i prompts/catalog_images.md --image ./img1.png --image ./img2.png`
**Expect:** LLM processes and references both images in response
