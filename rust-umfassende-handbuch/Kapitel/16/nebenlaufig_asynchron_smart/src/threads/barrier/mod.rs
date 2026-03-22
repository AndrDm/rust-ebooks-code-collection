#![cfg(feature = "thread_barrier")]

use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

pub fn main() {
    const THREAD_BARRIERE: usize = 5;
    let barriere = Arc::new(Barrier::new(THREAD_BARRIERE));
    let mut thread_gruppe = Vec::new();

    for i in 1..=THREAD_BARRIERE {
        let barriere_klon = barriere.clone();
        thread_gruppe.push(thread::spawn(move || {
            println!("Thread #{i} wartet");
            let wait_result = barriere_klon.wait();
            println!(
                "Thread #{i} fertig. Ist Anführer: {}",
                wait_result.is_leader(),
            );
        }));

        // Es dauert, bis alle Ressourcen der Threads bereitstehen...
        thread::sleep(Duration::from_millis(800));
    }

    for thread_handle in thread_gruppe {
        let _ = thread_handle.join();
    }
}
