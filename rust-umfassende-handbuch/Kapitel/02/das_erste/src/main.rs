#[allow(unused_must_use)]

fn main() {
    println!("Hello, world!");

    ferris_says::say(
        // Die zu schreibende Zeichenkette
        "Hallo, Rust",
        // Die breite der Ausgabe in Spalten
        20,
        // Die Standardausgabe bereitstellen
        std::io::stdout()
    );
}
