#!/bin/bash


cargo +stable build --release

cp target/release/*.a ./bin

gcc -o zkp zkp.c -L./bin -lzkp -I./include -lm 

#cbindgen --config cbindgen.toml --output include/zkp.h