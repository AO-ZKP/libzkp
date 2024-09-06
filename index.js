const fengari = require('fengari');
const lua = fengari.lua;
const lauxlib = fengari.lauxlib;
const lualib = fengari.lualib;

// Import the WASM module
const groth16_wasm = require('./pkg/groth16_wasm.js');

function main() {
    // Access the WASM exports directly
    const wasm = groth16_wasm.__wasm;

    console.log("WebAssembly exports:", Object.keys(wasm));

    // Create a new Lua state
    const L = lauxlib.luaL_newstate();
    
    // Load Lua standard libraries
    lualib.luaL_openlibs(L);

    // Define a Lua function that uses our WASM function
    lua.lua_pushcfunction(L, (L) => {
        const a = lauxlib.luaL_checkinteger(L, 1);
        const b = lauxlib.luaL_checkinteger(L, 2);
        console.log("Inputs:", a, b);
        const result = wasm.add_numbers(Number(a), Number(b));
        console.log("WASM result:", result);
        lua.lua_pushinteger(L, result);
        return 1;
    });
    lua.lua_setglobal(L, 'add_numbers');

    // Run some Lua code
    const luaCode = `
        local result = add_numbers(5, 7)
        print("Lua result:", result)
    `;

    if (lauxlib.luaL_dostring(L, fengari.to_luastring(luaCode)) !== lua.LUA_OK) {
        console.error(fengari.to_jsstring(lua.lua_tostring(L, -1)));
    }

    // Close the Lua state
    lua.lua_close(L);
}

main();