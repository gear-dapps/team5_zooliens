.PHONY: all build fmt init lint pre-commit test full-test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace -- -D warnings
	@cargo +nightly clippy \
	--all-targets \
	--workspace \
	-Fbinary-vendor \
	-- -D warnings

pre-commit: fmt lint full-test

deps:
	@echo ⚙️ Downloading dependencies...
	@if [ ! -f "./target/nft-0.2.5.opt.wasm" ]; then wget "https://github.com/gear-dapps/non-fungible-token/releases/download/0.2.5/nft-0.2.5.opt.wasm" -O "./target/nft-0.2.5.opt.wasm"; fi

test:
	@echo ⚙️ Running unit tests...
	@cargo +nightly t -Fbinary-vendor

full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t -Fbinary-vendor -- --include-ignored --test-threads=1
