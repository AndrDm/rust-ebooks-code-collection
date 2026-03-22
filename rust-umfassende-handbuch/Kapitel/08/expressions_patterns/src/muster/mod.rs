pub fn main() {
    {
        // Tupel- und Array-Muster

        // Tupel-Muster
        let (x, y) = (5, 8);
        println!("Koordinaten: {x}, {y}");

        // Array-Muster
        let [wort_1, wort_2, wort_3, ..] = ["Hallo", ",", "Rust", "!", "   "];
        println!("{wort_1}{wort_2} {wort_3}");
    }

    {
        // struct-Muster
        struct Person {
            name: String,
            alter: i32
        }

        let person_1 = Person { name: "Fred".to_string(), alter: 45 };
        // Eine Struktur zerlegen
        let Person { name, alter, .. } = &person_1;
        println!("Name: {name}, Alter: {alter}");
    }

    {
        // Nur das Bezeichner-Muster bindet an jeden Wert

        // Der Bezeichner bindet immer
        let x = 10;

        // Nicht idiomatisch, aber hier können auch Klammern stehen
        let (y) = x;

        // Fehler ein Tupel-Muster erwartet ein Tupel
        // let (a, b) = ["Hallo", "Rust"];

        // Fehler, auch der Typ des Tupels muss passen
        // let (a, b) = ("Hallo", "Rust", "!");
    }

    {
        // Einfache Muster
        let zahl = 13;

        match zahl {
            // Literal-Muster
            1 => "Eins",
            2 => "Zwei",
            // Oder-Muster
            3 | 4 => "Drei oder vier",
            // Alle anderen mit dem Unterschrich einfangen
            _ => "Eine andere Zahl"
        };

        let moegliche_zahl: Result<i32, std::num::ParseIntError> = "1".parse();
        // Ausdrücke mit Mustern
        let ergebnis = match moegliche_zahl {
            // Nur Ok mit Wert 1
            Ok(1) => "Eins",
            // Alle anderen Zahlen durch "_" abdecken
            Ok(_) => "Eine Zahl",
            // Alle Fehlerfälle abdecken
            Err(_) => "Konnte nicht geparst werden"
        };
        println!("{ergebnis}");

        {
            // Referenzen im Muster

            struct Person {
                alter: i32
            }

            let mut p = Person { alter: 41 };

            if let mut alter = p.alter {
                // Kopie, keine Änderung im Original
                alter = 42;
                println!("Originalalter: {}", p.alter); // 41
            }

            {
                // Das &-Muster
                // Fehler, Muster erwartet nun eine Referenz
                // if let &mut alter = p.alter {
                //     // Kopie, keine Änderung im Original
                //     alter = 42;
                //     println!("Originalalter: {}", p.alter);
                // }

                // Das Muster passt, aber dennoch Copy!
                if let &mut mut alter = &mut p.alter {
                    // Kopie, keine Änderung im Original
                    alter = 42;
                    println!("Originalalter mit &mut mut: {}", p.alter); // 41
                }

                // Übersichtlicher, den Ausdruck ändern
                // und mit Bezeichner an alles binden
                if let alter = &mut p.alter {
                    // Endlich, das Original ist verändert
                    *alter = 42;
                    println!("Originalalter, nur Ausdruck: {}", p.alter);
                }
            }


            // Das Muster soll gegen i32 abgleichen,
            // dann aber den Wert als Referenz verarbeiten
            if let ref mut alter = p.alter {
                // Das Muster referenziert den Ausdruck
                *alter = 42;
                println!("Originalalter mit ref: {}", p.alter); // 42
            }
        }

        {
            // Elementbereich-Muster

            // Generiere eine Zufallszahl
            let x = rand::random::<i32>();

            let zahl_ist = match x {
                i32::MIN..=2 => format!("Kleiner 3"),
                3..=9 => format!("Kleiner 10"),
                _ => format!("Größer 10")
            };
            println!("{zahl_ist}");


            {
                let zahl = 1.0;

                // Achtung, Fließkommazahlen werden zukünftig entfernt
                match zahl {
                    std::f64::consts::PI => {},
                    0.0..=2.0 => {},
                    3.0 => {},
                    _ => {}
                }
            }

            {
                // Den Vorrang im Muster mit der Gruppe steuern
                let zahl = rand::random::<i32>();

                // match &zahl {
                //     // Fehler
                //     &i32::MIN..=-1 => println!("Negative Zahl"),
                //     // Fehler
                //     &0..=i32::MAX => println!("Positive Zahl")
                // }

                let zahl = rand::random::<i32>();
                match &zahl {
                    &(i32::MIN..=-1) => println!("Negative Zahl"),
                    &(0..=i32::MAX) => println!("Positive Zahl")
                }
            }
        }
    }

    {
        // Mehr zum Bezeichner Muster

        let x = rand::random::<i32>();
        let zahl_ist = match x {
            kleiner_3 @ i32::MIN..=2 => format!("Kleiner 3: {kleiner_3}"),
            kleiner_10 @ 3..=9 => format!("Kleiner 10: {kleiner_10}"),
            viel_groesser @ _ => format!("Größer 10: {viel_groesser}")
        };
        println!("{zahl_ist}");

        // Ein Muster an einen Bezeichner binden

        // Fehler, möglich, dass kein Wert passt!
        // let muster_in_zuweisung @ [a, .., b] = "abc".as_bytes();
        // println!("{muster_in_zuweisung:?}");

        // Fehler, möglich, dass kein Wert passt!
        // fn addiere(a @ 1..=5 : i32, b @ 6.. :i32 ) -> i32 {
        //     a + b
        // }
    }

    {
        // Widerlegbare und unwiderlegbare Muster

        // OK, das Bezeichner-Muster bindet an alles
        let a = 2;

        // Die Widerlegbarkeit greift nach der Typsicherheit
        // Unpassende Typen erkennt der Compiler schon vorher
        // let (a, b, c) = (1.4, 1.5); // Fehler

        // OK, Restmuster überspringt nur alles zwischen Anfang und Ende
        let (a, .., c) = (1.4, 1.5);

        let v: Vec<i32> = vec![];
        // Fehler, der Compiler kann nicht mehr helfen
        // Das Muster kann zur Laufzeit fehlschlagen
        // let [start, ende] = v.as_slice();

        // Richtig, mit else-Block den Fehler behandeln
        // let [start, ende] = v.as_slice() else {
        //     panic!();
        // };

        let zufallszahl = rand::random::<i32>();
        // Hier kein Unterstrich nötig, mögliche Fälle werden behandelt
        match zufallszahl {
            i32::MIN..=0 => println!("Zahl ist negativ"),
            // Alle anderen Fälle
            1.. => println!("Zahl ist positiv"),
        };

        let zeichen = rand::random::<char>();
        let zeichen_ist_ein = match zeichen {
            '!' => "Ist ein Ausrufezeichen".to_string(),
            '?' => "Ist ein Fragezeichen".to_string(),
            // Noch viele weitere Werte nicht abgedeckt, daher der _-Arm
            _ => "Ist etwas anderes".to_string()
        };
        println!("{zeichen_ist_ein}");
    }


    {
        // Muster in Funktionen

        {
            fn addiere(a: i32, b: i32) -> i32 {
                a + b
            }
        }

        {
            let str1 = "Hallo".to_string();
            let str2 = "Rust".to_string();

            // Strings per Move entgegennehmen,
            // aber gleichzeitig mit ref vor Veränderung schützen
            fn strings_freigeben(ref a: String, ref b: String) {
                // Ok
                println!("Wird freigegeben: {a} und {b}");

                // Fehler, a ist hinter einer geteilten Referenz
                // a.push_str(&b);
            }

            strings_freigeben(str1, str2);
            // str1 und str2 sind jetzt freigegeben
        }

        {
            // Das makro matches!

            let ergebnis = "1233".parse::<i32>();

            let erfolgreich = matches!(ergebnis, Ok(_));
            // true
            println!("matches: {erfolgreich}");

            let ist_vier_stellige_zahl = matches!(ergebnis, Ok(n) if n > 999);
            // false
            println!("matches 4-stellig: {ist_vier_stellige_zahl}");



            if let Ok(_) = ergebnis {
                //
                println!("if let Alternative");
            }


        }
    }
}