#![cfg(feature = "thread_starten_join")]

use std::thread;
use std::time::Duration;

pub fn main() {
    println!("Haupt-Thread: {:?}", thread::current().id());

    let join_handle = thread::spawn(|| {
        println!("Thread Id: {:?}", thread::current().id());
        thread::sleep(Duration::from_millis(200));
    });

    #[allow(dead_code)]
    let _ergebnis = join_handle.join();

    println!("Programmende");
}