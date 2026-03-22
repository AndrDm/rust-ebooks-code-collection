#![cfg(feature = "thread_condvar")]

use std::sync::{Condvar, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    let bedingung = Condvar::new();

    let mutex = Mutex::new(String::new());

    thread::scope(|scope| {
        scope.spawn(|| loop {
            let guard = mutex.lock().unwrap();
            let mut guard_str = bedingung.wait(guard).unwrap();
            *guard_str = "PING".into();
        });
        scope.spawn(|| loop {
            // Einen der beiden Threads aufwecken
            bedingung.notify_one();

            {
                // Ressource nach Zugriff auslesen
                let guard = mutex.lock().unwrap();
                println!("{}", *guard);
            } // <-- MutexGuard implizit freigeben

            // Nächster Durchlauf in einer Sekunde
            thread::sleep(Duration::from_secs(1));
        });
        scope.spawn(|| loop {
            let guard = mutex.lock().unwrap();
            let mut guard_str = bedingung.wait(guard).unwrap();
            *guard_str = "PONG".into();
        });
    });
}
