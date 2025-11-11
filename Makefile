fmt:
	cargo fmt --all

clippy:
	cargo clippy -- -D warnings

test:
	cargo test

help:
	cargo run -- help

credits:
	cargo run -- credits

run:
	cargo run -- $(ARGS)

check: fmt clippy test