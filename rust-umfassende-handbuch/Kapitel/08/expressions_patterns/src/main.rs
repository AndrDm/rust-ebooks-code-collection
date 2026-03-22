mod konditionale_ausdruecke;
mod schleifen;
mod operatoren;
mod muster;
mod ausdruecke;

fn main() {

    ausdruecke::main();
    operatoren::main();
    konditionale_ausdruecke::main();
    schleifen::main();
    muster::main();

    {
        // Beispiele für Anweisungen

        let x: i32;

        struct Person {}

        fn registriere_person(p: Person) { }
    }

    {
        // Ausdruck-Anweisung

        fn melde_ergebnis(a: i32, b: i32) -> i32 {
            let ergebnis = a + b;
            println!("Das Ergebnis ist: {ergebnis}");
            ergebnis
        }

        // Ausdruck-Anweisung, das Ergebnis der Funktion
        // wird ignoriert
        melde_ergebnis(2, 3);

        // Ausdruck innerhalb der let-Anweisung
        let aktuelles_ergebnis = melde_ergebnis(4, 4);
        // Mit der Rückgabe weiterarbeiten ...

        // Fehler, wohin mit dem Ergebnis?
        // 2 + 3

        // Ok, weil ignoriert
        2 + 3;

        // {
        //     println!("Berechne");
        //     // ...

        //     // Fehler, ändert den Rückgabetyp des Blocks
        //     // von Unit-Typ () auf i32
        //     melde_ergebnis(5, 2)
        // } // Folgefehler, das Ergebnis wird nicht gespeichert

        {
            println!("Berechne");
            // ...
            // Ändert den Rückgabetyp des Blocks
            // von Unit-Typ () auf i32. Aber...
            melde_ergebnis(5, 2)
        }; // ... Ok, ignoriere hier das Ergebnis des Blocks

        {
            // ...

            // Ok, Ergebnis schon hier ignorieren
            melde_ergebnis(5, 2);
        }
    }

    {
        // Anweisung und Ausdruck zusammen
        let mut x: i32 = 2;

        x = 3;

        let x: i32 = 2 + 2;

        let zahl_ist = match x {
            i32::MIN..=2 => "Kleiner 3",
            2..=9 => "Kleiner 10",
            _ => "Größer oder gleich 10"
        };
        println!("{zahl_ist}"); // "Kleiner 10"
    }

    {
        // Der Unit-Typ ist das Ergebnis der Zuweisung

        let x: i32;
        // Die Zuweisung weist keinen gültigen Wert auf...
        let y = x = 2;

        // x hat den Typ i32
        println!("{}", std::mem::size_of_val(&x)); // 4

        // y hat den Unit-Typ
        println!("{}", std::mem::size_of_val(&y)); // 0

        // ... das gilt auch für die Neuzuweisung
        let mut a = 1;
        let b = a = 2;

        // b hat den Unit-Typ
        println!("{}", std::mem::size_of_val(&b)); // 0
    }



    {
        // Aktuell ungenutzt !!!

        const HALLO: &str = "Hallo";
        const RUST: &str = "Rust";

        let str = HALLO;
        let erkannt = match str {
            HALLO => "Ist Hallo",
            RUST => "Ist Rust",
            _ => "Etwas anderes"
        };
        println!("{erkannt}");

        let zahl = rand::random::<i32>();
        const ZUFALLSWERT: i32 = 42;
        let zahl_ist = match zahl {
            i32::MIN => "Kleinster Wert",
            i32::MAX => "Größter Wert",
            ZUFALLSWERT => "Zufallswert",
            _ => "Etwas dazwischen"
        };
        println!("{zahl_ist}");


        // Der Unterstrichausdruck

        // Nehme a und c, ignoriere b
        let [a, _, c] = "abc".as_bytes() else {
            // Wenn das Muster nicht passt
            panic!();
        };

        println!("{a}, {c}"); // 97, 99
    }
}
