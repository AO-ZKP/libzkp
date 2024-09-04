#!/bin/bash

cargo build --release

# The resulting wasm file will be in target/wasm64-unknown-unknown/release/libgroth16_wasm.wasm

# The resulting wasm file will be in target/wasm64-unknown-unknown/release/libgroth16_wasm.a
cp target/release/libgroth16_wasm.a ./bin/