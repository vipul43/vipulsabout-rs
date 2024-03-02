RUSTUP:=$(shell which rustup)
CARGO:=$(shell which cargo)

setup:
	$(RUSTUP) component add rustfmt --toolchain nightly
	$(RUSTUP) component add clippy

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
	$(CARGO) build

build-prod bp:
	$(CARGO) build --release

run r: build
	RUST_LOG=debug $(CARGO) watch -x 'run --bin vipulsabout-rs'

ci c: lint-check fmt-check audit test

debug: ci build
	RUST_LOG=debug $(CARGO) run --bin vipulsabout-rs

prod: ci build-prod
	RUST_LOG=info $(CARGO) run --bin vipulsabout-rs

clean:
	$(CARGO) clean
