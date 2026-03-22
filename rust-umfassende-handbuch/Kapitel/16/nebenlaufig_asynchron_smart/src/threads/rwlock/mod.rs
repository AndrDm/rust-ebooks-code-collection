#![cfg(feature = "thread_rwlock")]

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub fn main() {
    let rw_lock = Arc::new(RwLock::new("Hallo, Rust".to_string()));

    let lock_klon = rw_lock.clone();
    thread::spawn(move || loop {
        let str = lock_klon.read().unwrap();
        println!("Gelesen 1: {str}");
        thread::sleep(Duration::from_millis(1500));
    });

    let lock_klon = rw_lock.clone();
    thread::spawn(move || loop {
        let str = lock_klon.read().unwrap();
        println!("Gelesen 2: {str}");

        thread::sleep(Duration::from_millis(400));
    });

    let lock_klon = rw_lock.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        let mut str = lock_klon.write().unwrap();
        *str += " Hallo, Rust!".into();
        println!("Geschrieben: {str}");
    })
    .join()
    .unwrap();
}
