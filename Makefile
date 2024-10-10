# Execute tests
test:
	cargo test

# Execute Clippy
clippy:
	cargo clippy

# Command to execute the game without seed
start:
	cargo run start

# Command to execute the game with a certain seed
seed:
	cargo run start $(coords)

# Default rule
.PHONY: test clippy
