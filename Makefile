# Variables
BINARY_NAME := cookie_parser
SRC := src
LIB := lib.rs
MAIN := main.rs

# Commands
run:
	@cargo run -- $(ARGS)

build:
	@cargo build --release

test:
	@cargo test -- --nocapture

fmt:
	@cargo fmt

lint:
	@cargo clippy

clean:
	@cargo clean

help:
	@echo "Available commands:"
	@echo "  run         - Run the project's CLI"
	@echo "  build       - Build the production version of the project"
	@echo "  test        - Run testing"
	@echo "  fmt 				 - Format the code (rust fmt)"
	@echo "  lint        - Lint the code (rust clippy)"
	@echo "  clean       - Artifacts cleanup"