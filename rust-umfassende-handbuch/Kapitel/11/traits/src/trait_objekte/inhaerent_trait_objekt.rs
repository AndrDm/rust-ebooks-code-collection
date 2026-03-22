

trait Output {
    // Methode, die auch auf dem
    // implementierenden Datentyp sichtbar ist
    fn write(&self, str: String) {
        // ...
    }
}

// Inhärente Implementierung des Trait-Objekts
impl dyn Output {

    // Diese Methode ist nur auf
    // Trait-Objekten sichtbar
    fn kombiniere(&self, str: &str) {
        println!("Hallo {str}");
    }
}

struct Console;

impl Output for Console { }

pub fn main() {
    // Der implementierende Datentyp
    let console = Console;

    // ... in das Trait-Objekt beugen
    let output: &dyn Output = &console;

    // Ok, nur auf Trait-Objekt sichtbar
    output.kombiniere("Rust!");

    // Ok, Trait-Funktion ist für den
    // implementierenden Datentyp sichtbar
    console.write("Hallo, Rust!".into());

    // Fehler! "kombiniere" aber nicht!
    // console.kombiniere("Oh Oh");

    // Fehler! Auch kein Zugriff über das Trait
    // <Console as Output>::kombiniere(console, "Oh Oh");

    // Ok, assoziierte Methode auf dem Trait
    <Console as Output>::write(&console, "Hallo, Rust!".into());
}