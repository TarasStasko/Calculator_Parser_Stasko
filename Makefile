run:
	cargo run

test:
	cargo test

run-file:
	cargo run -- --file expression.txt

help:
	cargo run -- --help

credits:
	cargo run -- --credits

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

doc:
	cargo doc --no-deps --open