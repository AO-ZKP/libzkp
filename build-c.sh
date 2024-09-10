#!/bin/bash


cargo +stable build --release

cp target/release/*.a ./bin

gcc -o groth16_wasm groth16_wasm.c -L./bin -lgroth16_wasm -I./include -lm 

#cbindgen --config cbindgen.toml --output include/groth16_wasm.h