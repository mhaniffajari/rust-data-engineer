PROJECT_DIRS := project/data_structure project/fruit-salad

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo fmt --quiet; \
	done

lint:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo clippy --quiet; \
	done

test:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo test; \
	done

run:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo run; \
	done

release:
	for dir in $(PROJECT_DIRS); do \
		cd $$dir && cargo build --release; \
	done

all: format lint test run
