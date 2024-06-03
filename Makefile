help:
	@echo make install
	@echo make list-modules
	@echo make list-dependencies
	@echo ""
	@echo make lint
	@echo make build
	@echo make format
	@echo make run
	@echo make test
	@echo make docs
	@echo make clean

install: rustup-install cargo-install

rustup-install:
	rustup component add rustfmt
	rustup componnent add clippy

cargo-install:
	cargo install cargo-modules
	cargo install cargo-tree

list-modules:
	cargo-modules structure

list-dependencies:
	cargo-modules dependencies

lint:
	cargo clippy

build:
	cargo build

format:
	cargo fmt

run:
	cargo run

test:
	cargo test

docs:
	cargo doc --open

clean:
	rm -fr ./target
