use std::thread::{Builder, sleep, scope, spawn};

fn main() {
    // Ein Panic im neuen Thread erzwingen
    let _ = Builder::new()
        .name("Nebenthread".to_string())
        .spawn(|| panic!());

    // Darauf warten, dass die Panic auftrat
    sleep(std::time::Duration::from_secs(2));

    println!("1. Panic");

    // Der Nebenthread isoliert die Scope-Panic
    spawn(|| {
        scope(|s| {
            let _ = Builder::new()
                .name("Scope Thread".to_string())
                .spawn_scoped(s, || {
                    println!("Hallo aus Scope Thread");
                    panic!();
                });
        });

        sleep(std::time::Duration::from_secs(1));
    });

    // Dieses Mal erscheint die Ausgabe
    println!("2. Panic");
}
