rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	cd /workspaces/rust-data-engineer/project/data_structure && cargo fmt --quiet

lint:
	cd /workspaces/rust-data-engineer/project/data_structure && cargo clippy --quiet

test:
	cd /workspaces/rust-data-engineer/project/data_structure && cargo test --quiet

run:
	cd /workspaces/rust-data-engineer/project/data_structure && cargo run

release:
	cd /workspaces/rust-data-engineer/project/data_structure && cargo build --release

all: format lint test run
