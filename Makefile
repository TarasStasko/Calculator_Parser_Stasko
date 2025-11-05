run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

doc:
	cargo doc --no-deps --open