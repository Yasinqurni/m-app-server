# Basic project information
APP_NAME=my_app

# Default target
.DEFAULT_GOAL := help

## Build the project in debug mode
build:
	cargo build

## Build the project in release mode
release:
	cargo build --release

## Run the project
run:
	cargo run

## Run the project with arguments (use: make run-args ARGS="arg1 arg2")
run-args:
	cargo run -- $(ARGS)

## Run tests
test:
	cargo test

## Run benchmarks (if using criterion or similar)
bench:
	cargo bench

## Run linter and formatting checks
lint:
	cargo clippy --all-targets --all-features -- -D warnings
	cargo fmt -- --check

## Format code
fmt:
	cargo fmt

## Clean build files
clean:
	cargo clean

## Watch for changes and rebuild (requires cargo-watch)
watch:
	cargo watch -x run

## Show help (this message)
help:
	@echo "Makefile commands:"
	@grep -E '^##' Makefile | sed -e 's/## //'