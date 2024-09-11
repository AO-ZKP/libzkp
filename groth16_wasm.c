#include <stdio.h>
#include "groth16_wasm.h"
#include "libgroth16_wasm.h"
// #include <emscripten.h>


// EMSCRIPTEN_KEEPALIVE
int main() {
    int result = poseidon_parse_prove_verify();
    
    printf("poseidon_parse_prove_verify: %d\n", result);
    
    return 0;
}