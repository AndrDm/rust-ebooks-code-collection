#![allow(unused_variables)]

fn main() {
    {
        let slice_1: &str = "Hallo";
        let slice_2: &str = "Rust";

        println!(
            "Adresse: {:p}, Länge: {}",
            slice_1.as_ptr(),
            slice_1.len()
        );
        // Adresse: 0x7ff7def103b8, Länge: 5
    }

    {
        // Kasten: Achtung Bytes, nicht Buchstaben
        let slice_1: &str = "Füße";

        println!(
            "Länge von 'Füße': {}",
            slice_1.len()
        );
        println!(
            "Zeichen in 'Füße': {}",
            slice_1.chars().count()
        );
        // Zeichen in 'Füße': 4
    }

    {
        // Ein Slice von einem statischen String
        let slice: &'static str = "Hallo";
    }

    {
        // Ein Slice aus einem String
        let string = String::from("Hallo");

        let slice_deref: &str = &string;

        let slice_as_ref: &str = string.as_ref();
    }

    {
        // Byte- und Raw String Slices
        let bytes = b"abc 123";
        println!("Byte Slice: {:?}", bytes);
        // Byte Slice: [97, 98, 99, 32, 49, 50, 51]

        {
            // Experiment: Das eigene toUpper
            let mut bytes = b"abc".clone();
            println!("Byte Slice vor Konvertierung: {:?}", bytes);
            // Byte Slice vor Konvertierung:  [97, 98, 99]

            bytes
                // Iteriere über Bytes mit exklusiver Referenz (mut)
                .iter_mut()
                // und drehe jedes 6. Bit um
                .for_each(|b| *b ^= 0b00100000_u8);
            println!("Byte Slice nach Konvertierung: {:?}", bytes);
            // Byte Slice nach Konvertierung: [65, 66, 67]

            let konvertierte_bytes = bytes
                .iter()
                .fold(
                    String::new(),
                    |mut str, b| {
                        str.push(char::from(b.clone()));
                        str
                    }
                );
            println!("Konvertiert: {konvertierte_bytes}");
        }


        {
            // Fehler, Zeichen müssen ASCII sein
            // let bytes = b"Füße";
        }

        {
            // Kasten: Traits Display und Debug
            let zahl = 42;
            println!("{zahl}"); // 42
            println!("{zahl:x}"); // 0x2a
            println!("{zahl:X}"); // 0x2A
            println!("{zahl:?}"); // 40
            println!("{:p}", &zahl); // 0x3c2a3ff544
        }

        {
            // Raw String Slices
            let pfad = r"C:\users\rust\downloads";
            // ...

            let ein_text = r#"In "Anführungszeichen" "#;

            let sondertext = r##" Seltener, aber möglicher #"Problemfall" oder auch "# "##;
            // ...
            let text = r####" Das können Sie immer weiter spielen: "###" "####;
        }

        {
            // Konkatenieren
            const BASIS_URL: &str = "http://dev.hallo.rust";
            const LOGIN: &str = "/login";

            // Fehler, &str implementiert nicht das Trait "Add"
            // let url = BASIS_URL + LOGIN;

            // Alternativen
            // Allokiert einen String im Heap
            let url = BASIS_URL.to_string() + LOGIN;
            // Allokiert ebenfalls einen String im Heap
            let url = format!("{BASIS_URL} {LOGIN}");

            // Stattdessen
            let url = concat!("http://dev.hallo.rust", "/login"); // Nur Literale

            // Wenn mit Variablen, dann das Crate
            use const_format::{concatcp, formatcp};

            let url: &str = formatcp!("{BASIS_URL}{LOGIN}"); // Statisches String Slice
            let url: &str = concatcp!(BASIS_URL, LOGIN); // Statisches String Slice
        }

    }
}