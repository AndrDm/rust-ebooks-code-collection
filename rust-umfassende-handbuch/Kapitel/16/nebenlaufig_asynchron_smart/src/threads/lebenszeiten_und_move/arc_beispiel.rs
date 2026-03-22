#![cfg(feature = "thread_arc_beispiel")]

use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let str = Arc::new("Hallo, Rust".to_string());
    let klon = str.clone();

    thread::spawn(move || {
        println!("Neben-Thread: {klon}");
        thread::sleep(Duration::from_millis(400))
        // <-- "klon" freigeben
    })
    .join()
    .unwrap();

    println!("Haupt-Thread: {str}");
    thread::sleep(Duration::from_millis(400));
    // <-- "str" freigeben
}
