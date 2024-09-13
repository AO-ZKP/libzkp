#!/bin/bash


# cargo +stable build --release

# cp target/release/*.a ./bin


emcc -o groth16.js groth16.c -L./bin -l:libgroth16_wasm.a -I./include -lm -s MEMORY64=1 -s WASM=1 -s SUPPORT_LONGJMP=1

#emcc -s MEMORY64=1 -Wno-experimental -c groth16.c -o groth16.o -o groth16.js -I ./include

#cbindgen --config cbindgen.toml --output include/groth16.h