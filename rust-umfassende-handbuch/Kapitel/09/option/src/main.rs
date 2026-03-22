#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    {
        // Den Wert untersuchen

        let option = Some(42);

        let hat_wert = option.is_some(); // true
        let ist_none = option.is_none(); // false

        // ...

        let option: Option<i32> = None;

        let hat_wert = option.is_some(); // false
        let ist_none = option.is_none(); // true

        let option = Some(42);

        let ist_ungerade = option
            .filter(|wert: &i32| wert.is_negative())
            .is_some(); // false
        println!("Ist ungesetzt: {ist_ungerade:?}");
    }

    {
        // Option als Iterator
        let zahlen = [1, 2, 3];
        let zahl = zahlen.get(2); // Ungültiger Index

        for wert in zahl {
            // Führt nur aus, wenn Some(T)
            println!("Die Zahl ist; {wert}");
        }

        {
            // Variante mit if-let
            let zahlen = [1, 2, 3];
            let zahl = zahlen.get(4); // Ungültiger Index

            if let Some(wert) = zahl {
                println!("Die Zahl ist; {wert}");
            }
        }

        let als_string: Option<String> = zahl.map(|z| (z * z).to_string());
        if let Some(wert) =  als_string {
            println!("Die Zahl quadriert und als String: {wert}");
        }
    }

    {
        // Den Wert entnehmen
        let option = Some(42);

        // Wie bei Result<T, E> auch verfügbar:
        // - unwrap_or
        // - unwrap_or_else
        // - unwrap_or_default
        let zahl = option.unwrap();

        let option = Some(42);
        let zahl = option.expect("Immer gültig, wegen Literal");

        // Beachten Sie, dass "option" veränderlich sein muss
        let mut option = Some(42);
        let zahl = option.take();
        println!("{zahl:?}, {option:?}"); // Some(42), None

        let zahl = option.get_or_insert(1);
        println!("{zahl}"); // 1

        option = None;

        // Eine Zufallszahl mit dem Crate rand erzeugen
        let zahl_rand = option.get_or_insert_with(|| rand::random::<i32>());
        println!("{zahl_rand}"); // 1507351660

        let neuer_wert = option.insert(21); // 21

        let alter_wert = option.replace(22);
        println!("{alter_wert:?}"); // Some(21)

        let ist_gesetzt = option.is_some(); // true
        println!("Ist gesetzt: {ist_gesetzt}");
    }

    {
        // Option<T> und der Fragezeichen-Operator

        fn addiere(a: Option<i32>, b: Option<i32>) -> Option<i32> {
            Some(a? + b?)
        }

        let ergebnis = addiere(Some(42), None); // None
        // ...
        let ergebnis = addiere(Some(21), Some(21)); // Some(42);
        println!("Ergebnis ist: {ergebnis:?}");
    }

    {
        // Ineinander verschachtelte Options abflachen
        let doppeltes_option = Some(Some(32));
        let ergebnis = doppeltes_option.flatten(); // Some(32)
        println!("Flattened Ergebnis: {ergebnis:?}");

        let doppeltes_option = Some(Some(Some(32)));
    }

    {
        // zip und unzip

        let worte = Vec::from(["Hallo", ",", "Rust"]);

        let str_1 = worte.get(0);
        let str_2 = worte.get(3);

        let ergebnis = str_1.zip(str_2); // None
        println!("{ergebnis:?}");

        let str_1 = worte.get(0);
        let str_2 = worte.get(2);
        let ergebnis = str_1.zip(str_2); // Some(("Hallo", "Rust"))
        println!("{ergebnis:?}");

        let str = "Hallo, Rust";
        let ergebnis = Some((str.len(), str)).unzip(); // (Some(11), Some("Hallo, Rust"))
        println!("{ergebnis:?}");
    }

    {
        let worte: Vec<_> = ["Hallo", ",", "Rust"].into_iter().map(String::from).collect();

        mod externe_lib {
            pub struct Person {
                // Pflichtangabe
                nutzername: String,
                // Optional den echten Namen angeben
                name: Option<String>,
            }

            impl Person {
                pub fn new(nutzername: String, name: Option<String>) -> Person {
                    Person { nutzername, name }
                }

                pub fn name(&self) -> &Option<String> {
                    &self.name
                }
            }
        }

        let p = externe_lib::Person::new(
            "Rusty".to_string(),
            None,
        );

        let name = p.name();

        // Ok, Wert wird nicht herausbewegt (Muster)
        // Alternative: match-Ausdruck
        if let Some(name) = name {
            let name = name.to_string(); // Clone
            // mit String-Clone weiterarbeiten
        }

        // Achtung: unwrap und expect
        {
            // let name = name.unwrap(); // Fehler
            // let name = name.expect("Ist immer gesetzt"); // Fehler
        }

        // Lösung: as_ref()
        {
            // let name = name.as_ref().unwrap();
        }
    }

    {
        // Von Option zu Result
        let option: Option<i32> = None;
        let result_aus_option = option.ok_or("Fehler"); // Err("Fehler")
        println!("{result_aus_option:?}");

        // "option" wiederverwendbar, da Copy (None)
        let result_aus_option = option.ok_or_else(
            || format!("Fehlercode: {}", rand::random::<i32>())
        ); // Err("Fehlercode: 1417661746")
        println!("{result_aus_option:?}");

        let option = Some("Hallo, Rust".to_string());
        let result = option.ok_or(0); // Ok("Hallo, Rust")
        println!("{result:?}");

        // Neues "option", da oben verbraucht
        let option = Some("Hallo, Rust".to_string());
        // Anonyme Funktion wird nur bei Bedarf ausgeführt
        let result_aus_option = option.ok_or_else(
            || format!("Fehlercode: {}", rand::random::<i32>())
        ); // Ok("Hallo, Rust")
        println!("{result_aus_option:?}");

        let option: Option<Result<String, i32>> = Some(Ok("Hallo, Rust".to_string()));
        let umgedreht = option.transpose();
        // Ok(Some("Hallo, Rust"))
        println!("Transpose: {umgedreht:?}");
    }
}