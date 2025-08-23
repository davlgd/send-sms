# Makefile for send-sms workspace

CARGO := cargo
CRATE := crates/cli
.DEFAULT_GOAL := help

.PHONY: build release test clean validate install help

build:
	$(CARGO) build

release:
	$(CARGO) build --release

test:
	$(CARGO) test

clean:
	$(CARGO) clean

validate: 
	$(CARGO) fmt -- --check
	$(CARGO) clippy -- -D warnings
	$(CARGO) test

install: release
	$(CARGO) install --path ${CRATE}

help:
	@echo "Makefile commands:"
	@echo "  build        - Compile the project in debug mode"
	@echo "  release      - Compile the project in release mode"
	@echo "  test         - Run the test suite"
	@echo "  clean        - Remove target directory"
	@echo "  validate     - Check code formatting, linting, and run tests"
	@echo "  install      - Install the CLI tool"
	@echo "  help         - Show this help message"

