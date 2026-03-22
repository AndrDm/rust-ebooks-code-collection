use std::ffi::c_int;

#[no_mangle]
extern "C" fn addiere(a: c_int, b: c_int) -> c_int {
    a + b
}