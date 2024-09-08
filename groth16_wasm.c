#include <stdio.h>
#include "groth16_wasm.h"

int main() {
    int result = wasm_test();
    
    if (result == 1) {
        printf("Success: Proof generated and verified correctly.\n");
    } else {
        printf("Failure: Proof generation or verification failed.\n");
    }
    
    return 0;
}