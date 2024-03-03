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
	$(CARGO) add gloo
	$(CARGO) add yew-router

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

# Test
test t: test-backend test-frontend
test-backend tb:
	$(CARGO) test --target wasm32-unknown-unknown --target-dir vipulsabout-frontend/target --manifest-path vipulsabout-frontend/Cargo.toml --package vipulsabout-frontend
test-frontend tf:
	$(CARGO) nextest run --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend --config-file vipulsabout-backend/.config/nextest.toml

# Build
## Build:debug
build b: build-backend build-frontend
build-backend bb:
	$(CARGO) build --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend
build-frontend bf:
	$(TRUNK) build --config ./vipulsabout-frontend/Trunk.toml
## Build:prod
build-prod bp: build-backend-prod build-frontend-prod
build-backend-prod bbp:
	$(CARGO) build --release --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend
build-frontend-prod bfp:
	$(TRUNK) build --release --config ./vipulsabout-frontend/Trunk.toml

# Run
run r: run-backend run-frontend
run-backend rb: build-backend
	RUST_LOG=debug $(CARGO) watch -x 'run --bin vipulsabout-backend --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend'
run-frontend rf: build-frontend
	$(TRUNK) serve --config ./vipulsabout-frontend/Trunk.toml

# CI
ci c: lint-check fmt-check audit test

debug: ci build
	$(CARGO) run --bin vipulsabout-backend --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend
	$(TRUNK) serve --config ./vipulsabout-frontend/Trunk.toml

prod: ci build-prod
	$(CARGO) run --release --bin vipulsabout-backend --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend
	$(TRUNK) serve --config ./vipulsabout-frontend/Trunk.toml

clean:
	$(CARGO) clean --target-dir vipulsabout-backend/target --manifest-path vipulsabout-backend/Cargo.toml --package vipulsabout-backend
	$(TRUNK) clean --config ./vipulsabout-frontend/Trunk.toml
