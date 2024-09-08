#!/bin/bash

# this is for your local machine's architecture
# cargo build --release

# cp target/release/libgroth16_wasm.a ./bin/

# build command for wasm64
# Ensure you have the nightly toolchain installed
#rustup toolchain install nightly

# Add the wasm64-unknown-unknown target
#rustup target add wasm64-unknown-unknown --toolchain nightly

## ignore the error after this, the rustup is retarded

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
cargo +nightly build -Zbuild-std=std,panic_unwind,panic_abort --target=wasm64-unknown-unknown --release -Zbuild-std-features=panic_immediate_abort

# The resulting wasm file will be in target/wasm64-unknown-unknown/release/libgroth16_wasm.a
cp target/wasm64-unknown-unknown/release/libgroth16_wasm.a ./bin/


