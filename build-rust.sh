#!/bin/bash

cargo build --release --target wasm32-unknown-unknown

cp target/wasm32-unknown-unknown/release/groth16_wasm.wasm ./bin
