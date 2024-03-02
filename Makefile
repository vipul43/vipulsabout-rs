RUSTUP:=$(shell which rustup)
CARGO:=$(shell which cargo)
TRUNK:=$(shell which trunk)

setup:
	curl https://sh.rustup.rs -sSf | sh
	$(RUSTUP) component add rustfmt --toolchain nightly
	$(RUSTUP) component add clippy
	$(CARGO) install cargo-audit
	$(CARGO) install cargo-nextest --locked
	$(CARGO) install trunk --locked
	$(CARGO) install wasm-bindgen-cli

lint-fix lf:
	$(CARGO) +nightly clippy --fix -Z unstable-options --allow-staged --allow-dirty

lint-check lc:
	$(CARGO) clippy --all-targets --no-deps -- -Dwarnings -Dclippy::unwrap_used -Dclippy::panic -Dmissing_docs

fmt-fix ff:
	$(CARGO) +nightly fmt

fmt-check fc:
	$(CARGO) +nightly fmt -- --check

audit a:
	$(CARGO) audit

test t:
	$(CARGO) nextest run

build b:
	$(TRUNK) build

build-prod bp:
	$(TRUNK) build --release

run r: build
	$(TRUNK) serve

ci c: lint-check fmt-check audit test

debug: ci build
	$(TRUNK) serve

prod: ci build-prod
	$(TRUNK) serve

clean:
	$(CARGO) clean
	$(TRUNK) clean
