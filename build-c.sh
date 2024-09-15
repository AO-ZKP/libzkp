#!/bin/bash


# cargo +stable build --release

# cp target/release/*.a ./bin


emcc -o groth16.js groth16.c -L./bin -l:libgroth16_wasm.a -I./include -lm \
    -s MEMORY64=1 \
    -s ENVIRONMENT=node \
    -s EXPORTED_RUNTIME_METHODS='["ccall", "cwrap"]' \
    -s EXPORTED_FUNCTIONS='["_main", "_groth16_test"]' \
    -s MODULARIZE=1 \
    -s EXPORT_NAME="createGroth16Module" \
    -s ALLOW_MEMORY_GROWTH=1
#emcc -s MEMORY64=1 -Wno-experimental -c groth16.c -o groth16.o -o groth16.js -I ./include

#cbindgen --config cbindgen.toml --output include/groth16.h