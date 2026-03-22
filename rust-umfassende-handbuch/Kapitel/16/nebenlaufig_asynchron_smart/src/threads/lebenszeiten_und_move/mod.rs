#![cfg(feature = "thread_arc_beispiel")]
mod arc_beispiel;

use std::thread;
use std::time::Duration;

pub fn main() {
    arc_beispiel::main();

    let str = "Hallo, Rust";

    thread::spawn(|| {
        // println!("{str}");
    })
    .join()
    .unwrap();

    thread::sleep(Duration::from_millis(400))
}
