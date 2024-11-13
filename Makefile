PROJECT_NAME = npuzzle

.PHONY: all build run clean test

default: build

build:
	cargo build

run:
	cargo run

clean:
	cargo clean

fclean: clean

all: build run