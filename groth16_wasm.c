#include <stdio.h>
#include "groth16_wasm.h"
#include "groth16.h"
// #include <emscripten.h>


// EMSCRIPTEN_KEEPALIVE
int main() {
    int result = wasm_test();
    
    if (result == 1) {
        printf("Success: Proof generated and verified correctly.\n");
        return 1;
    } else {
        printf("Failure: Proof generation or verification failed.\n");
        return 0;
    }
    
    return 0;
}