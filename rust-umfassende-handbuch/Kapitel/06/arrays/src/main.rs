use std::ops::{Index, IndexMut};

fn main() {

    // Typ inferiert
    let zahlen = [1, 2, 3, 4];

    // Typ explizit
    let slices: [&str; 2] = ["Hallo", "Rust"];

    let guter_rat = ["üben"; 3];
    println!("{guter_rat:?}");
    // ["üben", "üben", "üben"]

    {
        // 2D Matrix
        let m1 = [
            [1, 2],
            [3, 4]
        ];

        let m2 = [
            [5, 6],
            [7, 8]
        ];

        // Das Crate ndarray einbinden
        let matrix_1 = ndarray::arr2(&m1);
        let matrix_2 = ndarray::arr2(&m2);

        let result = matrix_1.dot(&matrix_2);
        // [[19, 22], [43, 50]]
        println!("Matrixmultiplikation: {}", result.to_string());

    }

    {
        // Zugriff per Index
        println!("{}", slices[0]); // OK, Hallo
        println!("{}", slices[1]); // OK, Rust
        // println!("{}", slices[2]); // Fehler, kompiliert nicht

        {
            // Keine Compilerprüfung für Auffüllendes Literal
            println!("{}", guter_rat[0]); // OK, üben
            // println!("{}", guter_rat[10]); // Erst OK, aber Panic zur Laufzeit!
        }

        // Mit Array-Muster auslesen
        let [gruss, name] = slices;
        println!("{gruss}, {name}"); // Hallo, Rust
    }


    {
        const kapazitaet: usize = 4;
        const zahlen: [i32; kapazitaet] = [1, 2, 3, 4];

        const fn berechne_kapazitaet(eingabe: [i32; kapazitaet]) -> usize {
            eingabe.len()
        }

        let in_worten: [&str; berechne_kapazitaet(zahlen)] = [
            "eins",
            "zwei",
            "drei",
            "vier"
        ];

        zahlen
            .into_iter()
            .for_each(
                |zahl| {
                    println!("{zahl} in Worten: {}", in_worten[zahl as usize - 1]);
                }
            );
    }


    {
        let mut zahlen = [1, 2, 3, 4];
        zahlen[0] = 4;
        zahlen[1] = 3;
        zahlen[2] = 2;
        zahlen[3] = 1;
        println!("{zahlen:?}");
        // 4, 3, 2, 1
    }

    {
        // Array mit Move-Datentyp

        {
            // Indizierungs-Ausdruck
            let strings = ["Hallo".to_string(), "Rust".to_string()];
            {
                // Fehlerfall
                // let hallo = strings[0]; // Fehler, Move nicht möglich
                // let name = strings[1]; // Fehler, Move nicht möglich
            }


            {
                // Lösung 1: Mit Referenzen arbeiten
                let hallo = &strings[0]; // Fehler, Move nicht möglich
                let name = &strings[1]; // Fehler, Move nicht möglich
            }

            {
                // Lösung 2: IndexMut Trait
                let mut strings = ["Hallo".to_string(), "Rust".to_string()];

                let hallo = &mut strings[0];
                let name = &mut strings[1];
            }
        }

        {
            // Mit Muster auslesen
            let strings = ["Hallo".to_string(), "Rust".to_string()];
            {
                let [hallo, name] = strings;
                println!("{hallo}, {name}");

                // Fehler, der String wurde per Move bewegt
                // println!("{}", strings[0]);
            }

            {
                let strings = ["Hallo".to_string(), "Rust".to_string()];
                let [ref hallo, ref name] = strings;
                println!("{hallo}, {name}");

                println!("{}", strings[0]);
                println!("{}", strings[1]);
            }

            {
                let mut strings = ["Hallo".to_string(), "Rust".to_string()];
                let [ref mut hallo, ref mut name] = strings;
                println!("{hallo}, {name}");

                println!("{}", strings[0]);
                println!("{}", strings[1]);
            }

            {
                let mut array = ["Hallo".to_string(), "Rust".to_string()];
            }
        }
    }
}
