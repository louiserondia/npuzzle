PROJECT_NAME = npuzzle

.PHONY: all build clean test

all: build

build:
	cargo build --release

clean:
	cargo clean

fclean: clean

test:
	cargo test

run:
	cargo run -- --heuristic manhattan -g 3 -i 10000
