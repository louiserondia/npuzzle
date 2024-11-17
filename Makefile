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
	cargo run -- --heuristic manhattan --generate 3 --generate-complexity 10000
