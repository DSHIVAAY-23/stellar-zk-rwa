all: test build

build:
	cargo build --target wasm32-unknown-unknown --release

test:
	cargo test

optimize:
	soroban contract optimize --wasm target/wasm32-unknown-unknown/release/zk_verifier.wasm
