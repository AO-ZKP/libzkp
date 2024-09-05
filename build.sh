#!/bin/bash

# Build the Rust library
./build-rust.sh

# Compile C code with Rust library to WebAssembly
emcc -o groth16.js groth16_wasm.c \
    -L ./bin -I ./include \
    -s WASM=1 \
    -s EXPORTED_FUNCTIONS='["_main"]' \
    -s EXPORTED_RUNTIME_METHODS='["ccall", "cwrap"]' \
    -s ENVIRONMENT='node' \
    -s ALLOW_MEMORY_GROWTH=1 \
    -s INITIAL_MEMORY=16MB \
    --preload-file bin/groth16_wasm.wasm \
    -lm

echo "Build complete. Run 'node groth16.js' to execute the program."