trait Output {
    /// Komponenten erhalten eigenes Log-Tag
    fn debug_tag() {
        println!("[Komponente] [Log]")
    }

    /// Soll jede Ausgabeform definieren
    fn write(&self, str: String);
}

struct Console {
    // Datenfelder...
}

impl Output for Console {
    /// Überschreibt Output
    fn debug_tag() {
        println!("[Komponente] [Log] [Console]")
    }

    fn write(&self, str: String) {
        // Um die Ausgaben unterscheiden zu können
        println!("Output Console: {str}");
    }
}

struct File {}
impl Output for File {
    fn write(&self, str: String) {
        // ...
    }
}

impl Console {
    fn debug_tag() {
        println!("[Console]")
    }
}

pub fn main() {
    // Fehler, muss Implementierung angeben
    //Output::debug_tag();

    // Qualifizierter Pfad über den implementierenden Datentyp
    Console::debug_tag();
    // Ausgabe: [Console]

    // Vollständig qualifizierter Pfad über den implementierenden Datentyp
    <Console>::debug_tag();
    // Ausgabe: [Console]

    // Vollständig qualifizierter Pfad über Trait
    <Console as Output>::debug_tag();
    // Ausgabe: [Komponente] [Log] [Console]

    <File as Output>::debug_tag();
    // Ausgabe:: [Komponente] [Log]

    // Eine Instanz von Console erzeugen
    let console = Console {};

    // Die Methode aufrufen
    console.write("Erster Eintrag".to_string());

    impl Console {
        fn write(&self, str: String) {
            println!("Inhärente Implementierung: {str}");
        }
    }

    console.write("Eine Nachricht".to_string());

    Console::write(&console, "Zweiter Eintrag".to_string());

    <Console as Output>::write(&console, "Dritter Eintrag".to_string());
}
