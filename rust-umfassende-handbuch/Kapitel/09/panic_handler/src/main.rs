#![no_main]
#![no_std]

#[panic_handler]
fn panic_verarbeiten(info: &core::panic::PanicInfo) {
    // ...
}

// fn main() {
//     panic!("Hallo, Rust!");
// }
