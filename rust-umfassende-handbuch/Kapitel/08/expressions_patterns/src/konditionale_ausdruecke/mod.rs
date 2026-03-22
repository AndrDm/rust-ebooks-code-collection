use std::collections::HashMap;
use std::fmt::format;

pub fn main() {
    // if Ausdruck

    {
        let x = rand::random::<i32>();
        if x > 4 {
            // ...
        }

        if x == 1 {
            // ...
        } else if x == 2 {
            // ...
        } else if x == 3 {
            // ...
        } else {
            //
        }
    }

    {
        // Der if Ausdruck gibt einen Wert zurück
        let x = rand::random::<i32>();

        let ist_gerade = if x % 2 == 0 {
            println!("Zahl ist gerade");
            // ...
            true
        } else {
            println!("Zahl ist ungerade");
            // ...
            false
        };

        println!("Ist gerade: {ist_gerade}");
    }

    {
        // if let Ausdruck

        {
            let mut zahlen = Vec::from([1, 2, 3]);

            if let Some(zahl) = zahlen.get(2) {
                println!("Zahl am zweiten Index: {zahl}");
            }
        }

        {
            // auch else if let und else
            let mut zahlen = Vec::from([1, 2, 3]);

            //  ...
            zahlen.pop();

            if let Some(zahl) = zahlen.get(2) {
                println!("Zahl am zweiten Index: {zahl}");
            } else if let Some(zahl) = zahlen.first() {
                println!("Die erste Zahl: {zahl}");
            } else {
                println!("Array ist leider leer");
            }
        }
    }

    {
        // match Ausdruck
        {
            // Einführendes Beispiel

            // Eine Web-API antwortete mit diesem Code
            let http_antwort = 404;

            const FALLBACK_URL: &str = "www.example.com/fallback";

            let fallback = match http_antwort {
                404 => {
                    // Fall behandeln
                    // ...

                    println!("API nicht gefunden");
                    Some(format!("{FALLBACK_URL}/404"))
                }
                200 => {
                    println!("Alles gut");
                    None
                }
                _ => todo!(),
            };

            // Weiter ...
        }

        {
            // match Ausdruck mit einzelnen Anweisungen in Armen
            let http_antwort = 404;

            const FALLBACK_URL: &str = "www.example.com/fallback";

            let fallback = match http_antwort {
                404 => println!("API Nicht gefunden"),
                200 => println!("Alles gut"),
                _ => todo!(),
            };
        }

        {
            let gruss = "Hallo, Rust".to_string();
            match gruss {
                // Bindet wie "_" an alle Werte
                hallo => println!("{hallo}"),
            }

            let gruss = "Hallo, Rust".to_string();
            match gruss {
                // Bindet wie "_" an alle Werte
                mut hallo => {
                    hallo = hallo.to_uppercase();
                    println!("{hallo}");
                }
            }
        }

        {
            // Kasten String hat keine Muster
            {
                // Nur Muster möglich, keine Funktionsaufrufe
                let string = "Hallo, Rust".to_string();

                match &string {
                    // Fehler, &str ist ein anderer Datentyp!
                    // "" => println!("String ist leer"),
                    // Ok, das Bezeichner-Muster bindet an jeden Wert
                    str => println!("String ist {} Zeichen lang", str.len()),
                    // Ok, Bezeichner bindet an Muster
                    str @ _ => println!("String ist {} Zeichen lang", str.len()),
                }

                // ...
                // Dereferenzierung von String hingegen möglich
                match string.as_str() {
                    // OK!
                    "" => println!("String ist leer"),
                    // ...
                    str @ _ => println!("String ist {} Zeichen lang", str.len()),
                }
            }
        }

        {
            let x = rand::random::<i32>();
            match x {
                // Oder-Muster: Ok, aber welche Zahl war es?
                4 | 5 | 6 => println!("Vier, fünf oder sechs"),
                // Ein Bezeichner für den Joker
                andere_zahl @ _ => println!("Eine andere Zahl: {andere_zahl}"),
            }

            {
                // Bezeichner beim Oder-Muster in einem Arm
                let x = rand::random::<i32>();

                match x {
                    // Fehler, alle Oder-Ausdrücke müssen den Bezeichner "i" verwenden
                    // i @ 4 | i @ 5 | 6 => println!("i ist: {i}"),
                    // Ok, alle Oder-Unterausdrücke verwenden "j"
                    j @ 7 | j @ 8 | j @ 9 => println!("j ist: {j}"),
                    // Fehler, alle Oder-Ausdrücke müssen den gleichen Bezeichner verwenden
                    // k @ 10 | l @ 11 | m @ 12 => println!("??? ist: {k} oder {l} {m}?"),
                    // OK, "andere_zahl" nimmt den Wert des Jokers entgegen
                    andere_zahl @ _ => println!("Eine andere Zahl: {andere_zahl}"),
                }

                {
                    // Kasten: Mit dem Gruppen-Muster vereinfachen
                    let zahl = rand::random::<i32>();
                    match zahl {
                        // Fehler, alle Oder-Ausdrücke müssen den Bezeichner "i" verwenden
                        i @ (4 | 5 | 6) => println!("i ist: {i}"),
                        // Ok, alle Oder-Unterausdrücke verwenden "j"
                        j @ (7 | 8 | 9) => println!("j ist: {j}"),
                        // ...
                        // Fehler, alle Oder-Ausdrücke müssen den gleichen Bezeichner verwenden
                        // k @ 10 | l @ 11 | m @ 12 => println!("??? ist: {k} oder {l} {m}?"),
                        // OK, "andere_zahl" nimmt den Wert des Jokers entgegen
                        andere_zahl @ _ => println!("Eine andere Zahl: {andere_zahl}"),
                    }
                }
            }
        }

        {
            // Wenn ein Wert nicht Copy ist, wendet der Compiler Move an
            let gruss = "Hallo, Rust".to_string();
            match gruss {
                // Bindet wie "_" an alle Werte
                hallo => println!("{hallo}"),
            }

            // Fehler, der Wert wurde schon aus gruss herausbewegt
            // println!("Und noch mal: {gruss}");
        }

        {
            // Bei einem Speicherausdruck überträgt sich die Lebenszeit
            // des untersuchten Audrucks auf den Arm
            let x: i32 = 22;
            let mut y: &i32 = &0;
            match &x {
                ergebnis @ 22 => {
                    let mut z = 22;

                    y = &z;
                }
                _ => println!("Kein match"),
            }
            // OK, Variable im match-Arm übernimmt die Sichtbarkeit von "x"
            // println!("Ergebnis aus match: {y}");
        }

        {
            // Die Lebenszeit der temporären Variable
            // endet mit dem ausführenden Arm
            let x: i32 = 22;
            let y: &i32;

            match x + 1 {
                ergebnis @ 23 => {
                    y = &ergebnis;
                }
                _ => println!("Kein match"),
            }

            // Fehler, Referenz auf nun ungültige lokale Variable
            // println!("{y}");
        }
    }

    {
        // match guards
        let x = rand::random::<i32>();
        match x {
            // Das Muster passt auf jeden Wert.
            // Der match-Guard filter zusätzlich
            gerade @ _ if gerade % 2 == 0 => println!("Gerade: {gerade}"),
            ungerade => println!("Ungerade: {ungerade}"),
        }

        {
            // Kasten: break Ausdruck nur in Schleifen

            let gruss = "Hallo, Rust";
            match gruss {
                str => 'label_block: {
                    if str == "Hallo" {
                        println!("{str}, Rust");
                        // Bezieht sich auf den
                        // umgebenden Label-Block
                        break 'label_block;
                    }

                    println!("{str}");
                }
            }
        }
    }
}
