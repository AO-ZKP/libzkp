#!/bin/bash

# RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
# cargo +nightly build -Zbuild-std=std,panic_unwind,panic_abort --target=wasm32-unknown-unknown --release -Zbuild-std-features=panic_immediate_abort

# cargo build --target=wasm32-unknown-unknown --release

rm -rf bin include pkg        
mkdir -p bin include pkg


rustup run nightly cbindgen  --crate groth16_wasm --output include/groth16_wasm.h # --config cbindgen.toml 

# wasm-bindgen target/wasm32-unknown-unknown/release/groth16_wasm.wasm --out-dir ./pkg  --target nodejs

################################ NEW BUILD SCRIPT ################################

# High level command, may break
RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name groth16_wasm #-- --target wasm32-unknown-unknown -Z build-std=std,panic_unwind,panic_abort -Zbuild-std-features=panic_immediate_abort

cp target/wasm64-unknown-unknown/release/*.wasm ./bin
cp target/wasm64-unknown-unknown/release/*.a ./bin
#node .
node --experimental-wasm-memory64 index.js


