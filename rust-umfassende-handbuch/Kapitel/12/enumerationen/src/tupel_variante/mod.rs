pub fn main() {
    enum OptionPerson {
        KeineAngaben, // Wie None
        // Alter, Name
        Person(i32, String), // Tupel-Variante
    }

    let mut p1: OptionPerson = OptionPerson::KeineAngaben;

    let mut p2: OptionPerson = OptionPerson::Person(23, "Fred".to_string());
    let mut p3: OptionPerson = OptionPerson::Person(25, "Freda".to_string());

    // Daten löschen
    p2 = OptionPerson::KeineAngaben;
    p3 = OptionPerson::KeineAngaben;

    // Neue Angaben zu p1
    p1 = OptionPerson::Person(38, "Frank".to_string());

    // Fehler, Zugriff nicht möglich
    // let frank_alter = p1.0;
    // Fehler, Zugriff nicht möglich
    // let frank_name = p1.1;
}
