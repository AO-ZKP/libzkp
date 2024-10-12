#!/bin/bash

rm -rf bin pkg
mkdir -p bin include pkg

RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec -C panic=abort" \
cargo +nightly build -Zbuild-std=std,panic_abort --target=wasm32-unknown-emscripten --release -Zbuild-std-features=panic_immediate_abort

#wasm-opt target/wasm32-unknown-emscripten/release/zkp.wasm -O4 -o target/wasm32-unknown-emscripten/release/zkp.wasm
#cargo build --target=wasm32-unknown-emscripten --release




rustup run nightly cbindgen  --crate zkp --output include/zkp.h # --config cbindgen.toml 

#rustup run nightly wasm-bindgen target/wasm32-unknown-emscripten/release/zkp.wasm --out-dir ./pkg  --target nodejs

################################ NEW BUILD SCRIPT ################################

# High level command, not compatible,  breaking, actually this works better, but the above is more closer to ao flags
#RUSTFLAGS="--cfg=web_sys_unstable_apis -Z wasm-c-abi=spec" rustup run nightly wasm-pack build --target nodejs --out-name zkp -- --target wasm32-unknown-emscripten -Z build-std=std,panic_unwind,panic_abort -Z build-std-features=panic_immediate_abort




#cp target/wasm32-unknown-emscripten/release/*.wasm ./bin
cp target/wasm32-unknown-emscripten/release/*.a ./bin

#node --experimental-wasm-memory64 index.js

cp bin/*.a ../aos-zkp/container/src/groth16

cp include/*.h ../aos-zkp/container/src/groth16

# ../ao-rust-c-test/zkp.h

# rm  ../ao-rust/dev-cli/container/src/groth16/libzkp.a  

# cp include/zkp.h ../ao-c-test/

# cp bin/*.a ../ao-rust/dev-cli/container/src/groth16
# cp include/* ../ao-rust/dev-cli/container/src/groth16

# cd ../ao-rust/dev-cli/container/ && ./build.sh