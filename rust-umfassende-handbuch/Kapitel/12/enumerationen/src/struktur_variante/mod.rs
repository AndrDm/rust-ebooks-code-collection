pub fn main() {
    enum OptionPerson {
        KeineAngaben, // Wie None
        // Alter, Name
        Person { alter: i32, name: String }, // Struktur-Variante
    }

    let mut p1: OptionPerson = OptionPerson::KeineAngaben;

    let mut p2: OptionPerson = OptionPerson::Person {
        alter: 23,
        name: "Fred".to_string(),
    };
    let mut p3: OptionPerson = OptionPerson::Person {
        alter: 25,
        name: "Freda".to_string(),
    };

    // Daten löschen
    p2 = OptionPerson::KeineAngaben;
    p3 = OptionPerson::KeineAngaben;

    // Neue Angaben zu p1
    p1 = OptionPerson::Person {
        alter: 38,
        name: "Frank".to_string(),
    };
}
