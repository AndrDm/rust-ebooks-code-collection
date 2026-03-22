#![cfg(feature = "thread_channel_async")]

use std::sync::mpsc;
use std::thread;

pub fn main() {
    let (sender, receiver) = mpsc::channel::<String>();

    let sender_kopie = sender.clone();
    thread::spawn(move || {
        if let Err(_) = sender_kopie.send("Nachricht von Sender 1".into()) {
            // Fehler behandeln
        }
    });

    let sender_kopie = sender.clone();
    thread::spawn(move || {
        if let Err(_) = sender_kopie.send("Nachricht von Sender 2".into()) {
            // Fehler behandeln
        }
    });

    // Alle Instanzen von Sender freigeben
    drop(sender);

    let _ = thread::spawn(move || {
        loop {
            // Blockiert, so lange es einen Sender gibt
            // und keine Nachricht vorliegt.
            if let Ok(nachricht) = receiver.recv() {
                println!("Nachricht erhalten: {nachricht}");
            } else {
                // Channel wurde abgebaut, alle Sender freigegeben
                break;
            }
        }
    })
        // Mit dem Haupt-Thread warten, bis der Empfänger-Thread beendet wurde
        .join();
}
