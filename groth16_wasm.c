#include <stdio.h>
#include "zkp.h"
#include "libzkp.h"
// #include <emscripten.h>


// EMSCRIPTEN_KEEPALIVE
int main() {
    int result = poseidon_parse_prove_verify();
    
    printf("poseidon_parse_prove_verify: %d\n", result);
    
    return 0;
}