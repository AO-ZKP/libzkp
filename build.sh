#!/bin/bash

rm -rf bin pkg
mkdir -p bin include pkg

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" \
cargo +nightly build --target=wasm32-unknown-unknown --release -Zbuild-std=std,panic_unwind,panic_abort # -Zbuild-std-features=panic_immediate_abort

#wasm-opt target/wasm64-unknown-unknown/release/groth16_wasm.wasm -O4 -o target/wasm64-unknown-unknown/release/groth16_wasm.wasm
#cargo build --target=wasm32-unknown-unknown --release

#rustup run nightly cbindgen  --crate groth16_wasm --output include/groth16_wasm.h # --config cbindgen.toml 

rustup run stable wasm-bindgen target/wasm32-unknown-unknown/release/groth16_wasm.wasm --out-dir ./pkg  --target nodejs

################################ NEW BUILD SCRIPT ################################


## ##RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name groth16_wasm # -- --target wasm64-unknown-unknown -Z build-std=std,panic_unwind,panic_abort -Z build-std-features=panic_immediate_abort




cp target/wasm32-unknown-unknown/release/*.wasm ./bin
cp target/wasm32-unknown-unknown/release/*.a ./bin



rm -rf temp-obj && mkdir -p temp-obj && cd temp-obj && emar x ../bin/libgroth16_wasm.a && cd ..


node  index.js

# ../ao-c-test/groth16_wasm.h

rm  ../ao/dev-cli/container/src/groth16/libgroth16_wasm.a  

#cp include/groth16_wasm.h ../ao-c-test/

cp bin/* ../ao/dev-cli/container/src/groth16
#cp include/* ../ao/dev-cli/container/src/groth16

cd ../ao-rust/dev-cli/container/ && ./build.sh