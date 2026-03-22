#![cfg(feature = "thread_channel_sync")]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (sync_sender, receiver) = mpsc::sync_channel::<String>(0);

    thread::spawn(move || {
        sync_sender.send("Nachricht 1".into()).unwrap();
        println!("Nachricht 1 gesendet");

        sync_sender.send("Nachricht 2".into()).unwrap();
        println!("Nachricht 2 gesendet");

        sync_sender.send("Nachricht 3".into()).unwrap();
        println!("Nachricht 3 gesendet");

        sync_sender.send("Nachricht 4".into()).unwrap();
        println!("Nachricht 4 gesendet");
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        if let Ok(str) = receiver.recv() {
            println!("Verarbeite: {str}");
        } else {
            break;
        }
    })
    .join()
    .unwrap();
}
