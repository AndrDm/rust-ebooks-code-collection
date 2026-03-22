fn main() {
    {
        // Nur auf dem Stack
        let a = 10;

        // "s" bindet an einen String mit Puffer im Heap
        let s = String::from("Dynamischer Speicher");
    }

    {
        let zahl: i32;
        {
            // Rng aus dem Crate "rand" sichtbar machen
            use rand::Rng;
            let mut generator = rand::thread_rng();
            // number ist hier von außen sichtbar
            zahl = generator.gen();
        }

        // Außerhalb des Blocks sind "rand" und "generator" nicht sichtbar!
        // Nur "nachricht" und "zahl" aus dem gleichen oder vorherigen Bereich
        let nachricht = "Die nächste zufällige Zahl ist";
        println!("{nachricht}: {zahl}");
    }

    {
        let zahl: i32 = {
            // Rng aus dem Crate "rand" sichtbar machen
            use rand::Rng;
            let mut generator = rand::thread_rng();
            generator.gen()
        };

        let nachricht = "Die nächste zufällige Zahl ist";
        println!("{nachricht}: {zahl}");
    }

    {
        // Blöcke mit labels und break

        'block_label: {
            println!("Bis hierhin");
            break 'block_label;

            // das obere break verhindert die Ausführung
            println!("aber nicht weiter");
        }

        'block_label: {
            println!("Bis hierhin");

            // Fehler
            // break 'bezeichner_gleicher_ebene;

            'bezeichner_gleicher_ebene: {
                println!("Neues Sprungziel");
            }
        }
    }

    {
        // Ownership: a und b erhalten jeweils einen Wert
        let a = 10;
        let b = a;
        println!("a = {a}"); // 10
        println!("b = {b}"); // 10
    }
    {
        // String besitzt einen Buffer auf dem Heap.
        // Eine bitweise Kopie ist nicht möglich!
        let str = String::from("Hallo");
        let str_2 = str;

        println!("str_2: {str_2}"); // Hallo

        // Fehler! Das kompiliert so nicht
        // println!("str: {str}");
    }
}
