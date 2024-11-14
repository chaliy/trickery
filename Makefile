fmt:
	cargo fmt

readme:
	cargo run generate -i ./prompts/trickery_readme.md > README.md