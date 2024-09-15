#!/bin/bash

rm -rf bin pkg
mkdir -p bin include pkg

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm32-wasip1 --release -Zbuild-std-features=panic_immediate_abort

#wasm-opt target/wasm32-wasip1/release/groth16_wasm.wasm -O4 -o target/wasm32-wasip1/release/groth16_wasm.wasm
#cargo build --target=wasm32-wasip1 --release




rustup run nightly cbindgen  --crate groth16_wasm --output include/groth16_wasm.h # --config cbindgen.toml 

rustup run nightly wasm-bindgen target/wasm32-wasip1/release/groth16_wasm.wasm --out-dir ./pkg  --target nodejs

################################ NEW BUILD SCRIPT ################################

# High level command, not compatible,  breaking, actually this works better, but the above is more closer to ao flags
#RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name groth16_wasm -- --target wasm32-wasip1 -Z build-std=std,panic_unwind,panic_abort -Z build-std-features=panic_immediate_abort




#cp target/wasm32-wasip1/release/*.wasm ./bin
cp target/wasm32-wasip1/release/*.a ./bin

node --experimental-wasm-memory64 index.js

# ../ao-rust-c-test/groth16_wasm.h

rm  ../ao-rust/dev-cli/container/src/groth16/libgroth16_wasm.a  

cp include/groth16_wasm.h ../ao-c-test/

cp bin/*.a ../ao-rust/dev-cli/container/src/groth16
cp include/* ../ao-rust/dev-cli/container/src/groth16

cd ../ao-rust/dev-cli/container/ && ./build.sh