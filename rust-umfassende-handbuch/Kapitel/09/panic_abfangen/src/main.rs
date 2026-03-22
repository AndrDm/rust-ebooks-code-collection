fn main() {
    std::panic::set_hook(
        Box::new(
            |info| {
                if let Some(fehler) = info
                    .payload()
                    .downcast_ref::<String>() {
                    println!(
                        "Eine Panic ist aufgetreten: {fehler}"
                    );
                } else if let Some(nachricht) = info
                    .payload()
                    .downcast_ref::<&str>() {
                    println!("{nachricht}");
                }
            }
        )
    );

    for i in 0..3 {
        liefere_zufallszahl();

        if i == 1 {
            // Den Standard wiederherstellen
            let _ = std::panic::take_hook();
        }
    }

    {
        // catch_unwind und resume_unwind im Zusammenspiel
        let panic_result = std::panic::catch_unwind(|| {
            // Panic hier aufhalten, falls sie auftritt
        });

        // Cleanup in externer Library
        // ...

        // Jetzt weiter im Rust Programm...
        if let Ok(_) = panic_result {
            std::panic::resume_unwind(
                Box::new("Und weiter gehts")
            );
        }
    }

    {
        // Panic Payloads, die nicht Strings sind
        // std::panic::panic_any(42);
    }
}

fn liefere_zufallszahl() {
    let ergebnis = std::panic::catch_unwind(|| {
        let zufallszahl = rand::random::<i32>();
        if zufallszahl % 2 == 0 {
            zufallszahl
        } else {
            // Auf dieser Aufgabe baut das
            // nächste Beispiel auf
            if zufallszahl > 0 {
                panic!("{zufallszahl}");
            } else {
                panic!("Negativ");
            }
        }
    });

    // Angenommen, dass dieser Code vom
    // Konsumenten der Bibliothek ausgeführt wird
    match ergebnis {
        Ok(zahl) => println!("Die Zahl ist: {zahl}"),
        Err(_) => println!("Panic"),
    };
}
