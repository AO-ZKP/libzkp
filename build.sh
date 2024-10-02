#!/bin/bash

rm -rf bin pkg
mkdir -p bin include pkg

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm64-unknown-unknown --release -Zbuild-std-features=panic_immediate_abort

#wasm-opt target/wasm64-unknown-unknown/release/zkp.wasm -O4 -o target/wasm64-unknown-unknown/release/zkp.wasm
#cargo build --target=wasm32-unknown-unknown --release




#rustup run nightly cbindgen  --crate zkp --output include/zkp.h # --config cbindgen.toml 

#rustup run nightly wasm-bindgen target/wasm64-unknown-unknown/release/zkp.wasm --out-dir ./pkg  --target nodejs

################################ NEW BUILD SCRIPT ################################

## High level command, not compatible,  breaking, actually this works better, but the above is more closer to ao flags
#RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name zkp -- --target wasm64-unknown-unknown -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort




cp target/wasm64-unknown-unknown/release/*.wasm ./bin
cp target/wasm64-unknown-unknown/release/*.a ./bin

node --expose-gc --experimental-wasm-memory64 --max-old-space-size=32768 index.js

# ../ao-c-test/zkp.h

rm  ../ao/dev-cli/container/src/groth16/libzkp.a  

#cp include/zkp.h ../ao-c-test/

cp bin/*.a ../ao/dev-cli/container/src/groth16
#cp include/* ../ao/dev-cli/container/src/groth16

# cd ../ao/dev-cli/container/ && ./build.sh