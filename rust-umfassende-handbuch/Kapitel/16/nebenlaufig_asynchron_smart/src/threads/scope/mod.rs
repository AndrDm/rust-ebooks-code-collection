#![cfg(feature = "thread_scope")]

use std::time::Duration;

pub fn main() {
    println!("Öffne den Scope");

    let str = "Hallo, Rust".to_string();

    std::thread::scope(|scope| {
        scope.spawn(|| {
            std::thread::sleep(Duration::from_millis(250));
            println!("Thread 1: {str}");
        });

        // Der Thread-Builder bietet eine spezifische Funktion für Scopes an
        let _ = std::thread::Builder::new().spawn_scoped(&scope, || {
            println!("Thread 2: {str}");
        });

        scope.spawn(|| {
            println!("Thread 3: {str}");
        });
    });

    println!("Schließe den Scope");
}
