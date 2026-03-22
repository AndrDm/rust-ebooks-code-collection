#![cfg(feature = "thread_starten")]

use std::thread;

pub fn main() {
    println!("Haupt-Thread: {:?}", thread::current().id());

    thread::spawn(|| {
        println!("Thread Id: {:?}", thread::current().id());
    });

    println!("Programmende");
}