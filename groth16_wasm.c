#include <stdio.h>
#include "groth16_wasm.h"

int main() {
    printf("Hello from C!\n");
    
    int32_t result = return_one();
    printf("Result From Rust: %d\n", result);
    return 0;
}