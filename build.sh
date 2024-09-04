#!/bin/bash
./build-rust.sh

# for native build
# gcc -o main main.c -L./bin -lgroth16_wasm -I./include -lm
# echo "Compilation complete. Output: main.wasm"


## FOR WASM64 BUILD
# Then, compile the C code and link with the Rust library
# Set up the base command
cmd="emcc -O3"

# Add common flags
cmd+=" -g2 -s ASYNCIFY=1 -s MEMORY64=1 -s STACK_SIZE=41943040"
cmd+=" -s ASYNCIFY_STACK_SIZE=41943040 -s ALLOW_MEMORY_GROWTH=1"
cmd+=" -s INITIAL_MEMORY=83886080 -s MAXIMUM_MEMORY=17179869184"
cmd+=" -s WASM=1 -s MODULARIZE -s DETERMINISTIC=1 -s NODERAWFS=0"
cmd+=" -s FORCE_FILESYSTEM=1 -msimd128"

# Add assertions
cmd+=" -s ASSERTIONS=2"  # Changed from 1 to 2 for more verbose assertions

# Add exported functions and runtime methods
cmd+=" -s EXPORTED_FUNCTIONS=['_malloc','_main']"  # Combined into a single array
cmd+=" -s EXPORTED_RUNTIME_METHODS=['cwrap']"

# Add project-specific components
cmd+=" main.c"                  # Source file
cmd+=" -L./bin -lgroth16_wasm"  # Library path and library
cmd+=" -I./include"             # Include directory

# Add additional error checking
cmd+=" -s STACK_OVERFLOW_CHECK=2"
cmd+=" -s SAFE_HEAP=1"

# Set output files
cmd+=" -o groth16_wasm.wasm"
cmd+=" -o groth16_wasm.js"

# Execute the command
echo "Executing: $cmd"
eval $cmd
echo "Compilation complete. Output: groth16_wasm.wasm groth16_wasm.js"
