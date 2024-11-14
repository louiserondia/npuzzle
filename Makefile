PROJECT_NAME = npuzzle

.PHONY: all build clean test

all: build

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
