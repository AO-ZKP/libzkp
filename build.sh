#!/bin/bash

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
cargo +nightly build -Zbuild-std=std,panic_unwind,panic_abort --target=wasm64-unknown-unknown --release   # -Zbuild-std-features=panic_immediate_abort

rm -rf bin include pkg        
mkdir -p bin include pkg
cp target/wasm64-unknown-unknown/release/*.wasm ./bin
cp target/wasm64-unknown-unknown/release/*.a ./bin

rustup run nightly cbindgen  --crate groth16_wasm --output include/groth16_wasm.h # --config cbindgen.toml 

rustup run nightly wasm-bindgen target/wasm64-unknown-unknown/release/groth16_wasm.wasm --out-dir ./pkg  --target nodejs


## High level command, may break
##RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name groth16_wasm -- --target wasm64-unknown-unknown -Z build-std=std,panic_unwind,panic_abort
node  --experimental-wasm-memory64 index.js
