#!/bin/bash
./build-rust.sh

gcc -o main main.c -L./bin -lgroth16_wasm -I./include -lm