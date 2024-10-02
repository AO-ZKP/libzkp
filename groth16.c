#include <stdio.h>
#include "groth16_wasm.h"
#include "groth16.h"
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
int groth16_test(){
    int result = rust_test();
    return result;
}

// Add this line to ensure the runtime is initialized
EMSCRIPTEN_KEEPALIVE
int main() { return 0; }