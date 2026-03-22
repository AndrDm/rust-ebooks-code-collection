#![cfg(feature = "thread_atomar")]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

pub fn main() {
    let atomare_zahl = AtomicUsize::new(0);

    thread::scope(|scope| {
        scope.spawn(|| {
            atomare_zahl.store(42, Ordering::Relaxed);
        });

        scope.spawn(|| {
            atomare_zahl.store(22, Ordering::Relaxed);
        });

        scope.spawn(|| {
            let zahl = atomare_zahl.load(Ordering::Relaxed);
            println!("Relaxed Zahl: {zahl}");
        });
    });

    {
        // fetch
        let atomare_zahl = Arc::new(AtomicUsize::new(0));

        let zahl = atomare_zahl.clone();
        let thread_1 = thread::spawn(move || {
            let vorher = zahl.fetch_add(22, Ordering::Relaxed);
            println!("Thread 1: Wert vorher war: {vorher}");
        });

        let zahl = atomare_zahl.clone();
        let thread_2 = thread::spawn(move || {
            let vorher = zahl.fetch_add(42, Ordering::Relaxed);
            println!("Thread 2: Wert vorher war: {vorher}");
        });

        thread_1.join().unwrap();
        thread_2.join().unwrap();

        println!("Zahl: {}", atomare_zahl.load(Ordering::Relaxed));
    }

    {
        let atomare_zahl = Arc::new(AtomicUsize::new(7));
        let z = atomare_zahl.fetch_update(
            Ordering::Relaxed,
            Ordering::Relaxed,
            |z| {
            if z == 7 {
                return None;
            }

            Some(70)
        });
        println!(
            "Zahl: {z:?} war, ist jetzt {}",
            atomare_zahl.load(Ordering::Relaxed)
        );

        let atomare_zahl = Arc::new(AtomicUsize::new(7));

        let ordering_wenn_gleich = Ordering::Relaxed;
        let ordering_wenn_ungleich = Ordering::Relaxed;
        let z = atomare_zahl.compare_exchange(7, 70, ordering_wenn_gleich, ordering_wenn_ungleich); // Wird ersetzt, da erwarteter Wert
        println!(
            "Zahl: {z:?} war, ist jetzt {}",
            atomare_zahl.load(Ordering::Relaxed)
        );
    }
}
