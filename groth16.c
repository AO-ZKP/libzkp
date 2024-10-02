#include <stdio.h>
#include "groth16_wasm.h"
#include "groth16.h"
#include <emscripten.h>

EMSCRIPTEN_KEEPALIVE
int groth16_test(){
    int result = wasm_test();
    return result;
}

