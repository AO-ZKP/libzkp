// lib.rs
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn add_numbers(a: c_int, b: c_int) -> c_int {
    a + b
}