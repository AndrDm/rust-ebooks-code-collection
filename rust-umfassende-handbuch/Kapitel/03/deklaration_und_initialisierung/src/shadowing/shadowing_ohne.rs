// Input/Output
use std::io::stdin;

// String-Konverter Trait
use std::str::FromStr;

pub fn main() {

    // Einen String Puffer erstellen
    let mut alter_als_string = String::new();

    // ... und Zeile einlesen
    stdin().read_line(&mut alter_als_string);

    // ... dann Steuerzeichen (z.B. Enter-Taste) entfernen
    alter_als_string = alter_als_string.trim().to_string();

    // i32 aus einem String lesen, from_str
    // entnehmen wir aus std::str::FromStr oben
    let alter_result = i32::from_str(&alter_als_string);

    // Handelt es sich um eine Zahl?
    // Mit Result<i32, ParseIntError> prüfen
    let hat_fehler = alter_result.is_err();
    if hat_fehler {
        println!(
            "{alter_als_string} ist keine Zahl. \
                Bitte eine Zahl eingeben."
        );
        return;
    }

    // Die Zahl (i32) aus
    // Result<i32, ParseIntError> herausnehmen
    let alter = alter_result.unwrap();

    let ist_gerade = alter & 0x01 == 0;
    // Das Ergebnis ausgeben
    println!(
        "Das Alter {alter_als_string} ist: {}",
        if ist_gerade {
            "gerade"
        } else {
            "ungerade"
        },
    );
}