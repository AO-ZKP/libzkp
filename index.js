// Import the WASM module

console.log("\n\n         *************** STARTING TEST FOR GROTH16 ***************");
const groth16_wasm = require('./pkg/groth16_wasm.js');

function main() {
    

    // Access the WASM exports directly
    const wasm = groth16_wasm.__wasm;

    console.log("\nWebAssembly exports:", Object.keys(wasm));





    // // Create a new Lua state
    // const L = lauxlib.luaL_newstate();
    
    // // Load Lua standard libraries
    // lualib.luaL_openlibs(L);

    // // Define the groth16 module
    // const groth16Module = `
    // local M = {}
    
    // function M.wasm_test()
    //     return wasm_test()
    // end
    
    // return M
    // `;

    // // Register the WASM function in Lua
    // lua.lua_pushcfunction(L, (L) => {
        console.log("\nCalling wasm_test");
       // const result = wasm.wasm_test();
        console.log("\nWASM result:", result);
    //     lua.lua_pushinteger(L, result);
    //     return 1;
    // });
    // lua.lua_setglobal(L, 'wasm_test');

    // // Register the custom require function
    // lua.lua_pushcfunction(L, (L) => {
    //     const moduleName = fengari.to_jsstring(lauxlib.luaL_checkstring(L, 1));
    //     if (moduleName === "groth16") {
    //         // Load the module
    //         if (lauxlib.luaL_loadstring(L, fengari.to_luastring(groth16Module)) !== lua.LUA_OK) {
    //             return lua.lua_error(L);
    //         }
    //         // Call the module function
    //         if (lua.lua_pcall(L, 0, 1, 0) !== lua.LUA_OK) {
    //             return lua.lua_error(L);
    //         }
    //         return 1;
    //     }
    //     lua.lua_pushnil(L);
    //     return 1;
    // });
    // lua.lua_setglobal(L, 'require');

    // // Run some Lua code
    // const luaCode = `
    //     local groth16 = require("groth16")
    //     local result = groth16.wasm_test()
    //     if result == 1 then
    //         print("\\nGroth 16 test PASSED, result:", result, "\\n")
    //     else
    //         print("\\nGroth 16 test FAILED, result:", result, "\\n")
    //     end
    // `;

    // if (lauxlib.luaL_dostring(L, fengari.to_luastring(luaCode)) !== lua.LUA_OK) {
    //     console.error(fengari.to_jsstring(lua.lua_tostring(L, -1)));
    // }

    // // Close the Lua state
    // lua.lua_close(L);
}

main();