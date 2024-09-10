#!/bin/bash

# Navigate to the go_src directory
cd go_src

# Ensure Go modules are up to date
go mod tidy

# Compile the Go code into a shared library
go build -buildmode=c-shared -o libgroth16.so ./cmd

# Move the shared library to the project root
mv libgroth16.so ../

# Navigate back to the project root
cd ..

# Compile the C program and link it with the Go library
gcc -o groth16_test c_src/main.c -L. -lgroth16 -Wl,-rpath,.

echo "Build complete. Run ./groth16_test to execute the program."