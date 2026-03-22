use std::ffi::{c_char, CString};

extern "C" {
    fn printf(m: *const c_char);
}

fn main() {
    let c_string = CString::new("Hallo, Rust").unwrap();

    unsafe {
        // printf von C aufrufen
        printf(
            c_string.as_ptr()
        );
    }
}
