
# Makefile for Rust library project

# Variables
CARGO = cargo
TARGET = my_library

# Default target
.PHONY: all
all: build

# Build the library
.PHONY: build
build:
	$(CARGO) build

# Run tests
.PHONY: test
test:
	$(CARGO) test

# Clean the project
.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf ~/.cargo/.package-cache


# Run the library (if applicable)
.PHONY: run
run:
	$(CARGO) run




