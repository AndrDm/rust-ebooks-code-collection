pub fn main() {

    {
        // Der Gruppen-Ausdruck

        let ergebnis = 2 + 3 * 7; // 23
        println!("Ohne Gruppe: {ergebnis}");
        let ergebnis = (2 + 3) * 7; // 35
        println!("Mit Gruppe: {ergebnis}");
    }

    {
        // Operatoren sind links-assoziativ
        let ergebnis = (1 + 2 + 3) == ((1 + 2) + 3); // true
        println!("links-assoziativ: {ergebnis}");
    }
}