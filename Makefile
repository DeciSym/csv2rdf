
lint:
	cargo install  cargo-machete
	cargo fmt --check
	cargo machete
	cargo clippy --benches --tests --bins --no-deps --all-features

build:
	cargo build

test:
	cargo test

presubmit: lint test
