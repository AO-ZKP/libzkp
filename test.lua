local ffi = require("ffi")

-- Load the Rust library
local lib = ffi.load("bin/libzkp.so")  -- Adjust the path and filename as needed

-- Define the C function signature
ffi.cdef[[
    int add_numbers(int a, int b);
]]

-- Use the Rust function
local result = lib.add_numbers(5, 7)
print("Result:", result)