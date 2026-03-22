#![cfg(feature = "thread_mutex")]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn main() {
    {
        // Fehler, Mutex ohne Arc
        // let str = "Hallo, Rust".to_string();
        //
        // // Mutex schützt Ressource "str" (Move in den Mutex)
        // let mutex = Mutex::new(str);
        //
        // thread::spawn(move || {
        //     let guard = mutex.lock();
        //     // Mit Ressource arbeiten
        // });
        //
        // // Fehler, "mutex" schon in den vorherigen Thread  bewegt
        // thread::spawn(move || {
        //     let guard = mutex.lock();
        //     // Mit Ressource arbeiten
        // });
    }

    {
        // Mutex mit Arc
        let str = "Hallo, Rust".to_string();

        // Mutex schützt Ressource "str" (Move in den Mutex)
        let mutex = Arc::new(Mutex::new(str));

        thread::scope(|scope| {
            const THREAD_MAX: usize = 5;
            for i in 1..=THREAD_MAX {
                let mutex_klon = mutex.clone();
                if i == 3 {
                    thread::spawn(move || {
                        let guard = mutex_klon.lock();
                        // Eine panic tritt auf
                        panic!();
                    });

                    return;
                }

                scope.spawn(move || {
                    println!("Spawn Thread: {i}");
                    let guard_result = mutex_klon.lock(); // Arc::clone
                    println!("Lock erhalten in Thread: {i}");

                    match guard_result {
                        Ok(guard) => {
                            // Implementiert Deref.
                            // Ohne Referenz würde der String bewegt
                            let str = &*guard;
                            println!("Mutex-Klon {i}: {str}");

                            // Den letzten Thread
                            // ohne Pause durchwinken
                            if i < THREAD_MAX {
                                thread::sleep(Duration::from_secs(1));
                            }
                        }
                        Err(guard) => {
                            let str = guard.get_ref();
                            println!("Fehler: {guard:?} mit String: {str}");
                        }
                    }
                });
            }
        }); // <-- Blockiert den Haupt-Thread
    }
}
