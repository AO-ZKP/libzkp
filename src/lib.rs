// lib.rs
#[no_mangle]
pub extern "C" fn add_numbers(a: i64, b: i64) -> i64 {
    a + b
}