#![cfg(feature = "thread_once")]

use std::sync::{Arc, Once};
use std::thread;

pub fn main() {
    thread::scope(|scope| {
        let once = Arc::new(Once::new());

        let once_klon = once.clone();
        scope.spawn(move || {
            once_klon.call_once(|| {
                println!("Thread 1!");
            });
        });
        let once_klon = once.clone();
        scope.spawn(move || {
            once_klon.call_once(|| {
                println!("Thread 2!");
            });
        });
    });
}
