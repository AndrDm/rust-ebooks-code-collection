#![cfg(feature = "async_await")]
use std::{thread, time::Duration};

use tokio::time;
use tokio::runtime::Builder;

async fn schreibe() {
    print!("Hallo");

    async {
        println!(", Rust!");
    }
        .await;
}

async fn zaehler<const START: usize>() {
    use std::time::Duration;

    for i in START..START + 5 {
        println!("Zähler: {i}, Thread: {:?}", thread::current().id());
        time::sleep(Duration::from_secs(1)).await;
        // thread::sleep(Duration::from_secs(1));
    }
}

pub fn main() {
    println!("Haupt-Thread Id: {:?}", thread::current().id());

    let Ok(laufzeit) = Builder::new_multi_thread()
        .enable_all()
        .worker_threads(2)
        .build()
        else {
            // Konnte die Laufzeit nicht initialisieren
            panic!()
        };

    let mut join_handles = vec![];

    join_handles.push(laufzeit.spawn(schreibe()));
    join_handles.push(laufzeit.spawn(zaehler::<0>()));
    join_handles.push(laufzeit.spawn(zaehler::<5>()));
    join_handles.push(laufzeit.spawn(async {
        println!("Das ist das Ende. Thread: {:?}", thread::current().id());
    }));

    for handle in join_handles {
        let _ = laufzeit.block_on(handle);
    }
}