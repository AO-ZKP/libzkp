#include <stdlib.h>
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>
#include "groth16.h"


extern lua_State *wasm_lua_state;

// The wasm_test wrapper function
static int l_wasm_test(lua_State* L) {
    int result = rust_test();
    lua_pushinteger(L, result);
    return 1;
}

// The array of luaL_Reg containing the functions to be registered
static const luaL_Reg groth16_functions[] = {
    {"wasm_test", l_wasm_test},
    {NULL, NULL}
};

// The entry point function for the library
int luaopen_groth16(lua_State* L) {
    luaL_newlib(L, groth16_functions);
    return 1;
}