// Builder und die Funktionen scope und sleep im
// Gültigkeitsbereich bekannt machen
use std::thread::{Builder, scope, sleep};

fn main() {

    // Ein Panic im neuen Thread erzwingen
    let _ = Builder::new()
        .name("Nebenthread".to_string())
        .spawn(|| panic!());

    // Darauf warten, dass die Panic auftrat
    sleep(std::time::Duration::from_secs(2));

    println!("1. Panic");

    // Die Panic schlägt auf den Hauptthread durch
    scope(|s| {
        let _ = Builder::new()
            .name("Scope Thread".to_string())
            .spawn_scoped(s, || panic!());
    });

    // Diese Ausgabe wird nie erscheinen
    println!("2. Panic");
}
