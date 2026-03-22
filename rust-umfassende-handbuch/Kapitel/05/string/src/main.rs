use std::fmt::Arguments;
use std::ptr::addr_of;
// Implementierung von Typ Bestellung
use std::str::FromStr;

fn main() {
    {
        // String new und capacity
        let str = String::new();

        // Default Trait, ruft String::new() auf
        let str = String::default();

        let str = String::with_capacity(2048);
    }

    {
        let slice = "Ein String Slice";

        let str = slice.to_string();

        let str = slice.to_owned();

        let str: String = slice.into();

        let str = String::from(slice);
    }

    {
        let slice = "Ein String Slice";

        let bytes: Box<&str>;

        // OK...
        let str: String = slice.into();

        // ...auch mit anderem Typ OK
        bytes = slice.into();

        // Fehler!
        // let zahl: i32 = slice.into();
    }

    {
        // Parsen aus einem String
        let string = "42".to_string();
        let zahl: i32 = string
            .parse()
            .unwrap();

        {
            // FromStr im Sichtbarkeitsbereich einführen
            use std::str::FromStr;

            let string = "42".to_string();

            // from_str unten akzeptiert nur &str
            let slice = &*string;

            let zahl = i32::from_str(slice)
                .unwrap();
        }

        let float: f64 = string
            .parse()
            .unwrap();

        println!("i32: {zahl}, f64: {float:.2}");

        // Auch bool kann geparst werden
        let bool: bool = "true"
            .parse()
            .unwrap();
        println!("bool: {bool}");
    }

    {
        // Bearbeitungswerkzeuge
        let bestelltext = "1 x Pizza Salami";
        let bestellung: Bestellung = bestelltext
            .parse()
            .unwrap();

        println!("{bestellung:?}");

        let bestelltext = "1 x 77";
        let bestellung: Bestellung = bestelltext
            .parse()
            .unwrap();

        println!("{bestellung:?}");
    }

    {
        // Weitere Trim-Funktionen

        let string = "123abcZZZ";

        // Kann mit char-Slice verwendet werden
        let filter: &[char] = &['1', '2', '3', 'Z'];
        let gefiltert = string.trim_matches(filter);
        println!("Ausgabe nach matches: {gefiltert}");

        // Auch Prüf-Callback möglich
        let filter = |zeichen| {
            char::is_numeric(zeichen) || zeichen == 'Z'
        };

        let gefiltert = string.trim_matches(filter);
        println!("Ausgabe nach matches: {gefiltert}");
    }

    {
        // Formatierungen

        let gruss = "Hallo";
        let name = "Rust";

        // Positional
        let formatiert = format!("{}, {}", gruss, name);

        println!("{1}, {0} oder {}, {}", gruss, name);
        // Rust, Hallo oder Hallo, Rust

        // Fehler, gruss wird nicht verwendet
        //println!("{1}, {1} oder {1}, {1}", gruss, name);

        // Fehler, gruss wird nicht verwendet
        // let formatiert = format!("{0}, {}", gruss, name);


        // Interne Bezeichner im Formatierer
        let formatiert = format!(
            "{} {}, {}{}",
            ">", // Positional vor Named
            teil_1 = gruss,
            teil_2 = name,
            satzende = "!"
        );
        println!("{formatiert}");

        let formatiert = format!(
            "{teil_1}, {teil_2}",
            teil_1 = gruss,
            teil_2 = name
        );

        // Sie können auch mischen
        let formatiert = format!(
            "{teil_1}, {teil_2} oder {name}, {gruss}",
            teil_1 = gruss,
            teil_2 = name
        );
        // Hallo, Rust oder Rust, Hallo
        println!("{formatiert}");

        {
            // Kasten zu maskieren
            println!("{{Hier sind die Klammern maskiert}}{}", "!");
            // {Hier sind die Klammern maskiert}!
        }
    }

    {
        println!("{}", &2); // 2
        println!("{:p}", &2); // 0x7ff6f7093b28

        // Die Ausgabe transformatieren
        {
            let zahl = 5_f64 / 3_f64;
            println!("{zahl}"); // 1.6666666666666667
            println!("{zahl:.2}"); // 1.67
            println!("{:.4}", zahl); // 1.6667

            println!("{:.1$}", zahl, 4); // 1.6667
            // Oder nur die Anzahl als Argument
            println!("{zahl:.0$}", 4); // 1.6667
        }

        {
            // Padding
            let gruss = "Hallo";
            println!("|{gruss:^15}|");
            println!("|{gruss:<15}|");
            println!("|{gruss:>15}|");
            // |     Hallo     |
            // |Hallo          |
            // |          Hallo|

            println!("|{gruss:-^15}|");
            println!("|{gruss:-<15}|");
            println!("|{gruss:->15}|");
            // |-----Hallo-----|
            // |Hallo----------|
            // |----------Hallo|
        }

        {
            let zahl = 42.0;
            println!("{}", zahl); // 42
            println!("{:?}", [1, 2, 3]); // [1, 2, 3]
            println!("{:o}", 64); // 100
            println!("{:x}", 75); // 4b
            println!("{:X}", 75); // 4B
            println!("{:p}", &75); // 0x7ff6971a4de0
            println!("{:b}", 75); // 1001011
            println!("{:e}", 4200_000.0); // 4.2e6
            println!("{:E}", 4200_000.0); // 4.2E6
        }

        {
            // Das write! Makro

            use std::fmt::Write;
            let mut ausgabe = String::new();

            let _ = write!(
                &mut ausgabe,
                "Aus dem Stream: {}", true
            );
            println!("{ausgabe}");
            // Aus dem Stream: true

            let ausgabe = format!("Aus dem Stream: {}", true);
            println!("{ausgabe}");
            // Aus dem Stream: true

            {
                use std::io::Write;

                let mut ausgabe = Vec::new();

                let _ = write!(ausgabe, "{}{}{}", 1, 2, 3);
                println!("{ausgabe:?}");
                // [49, 50, 51]
            }
        }
    }
}

// Erweiterung aus Kapitel 6, "Collections und Slices"

#[derive(Debug)]
pub struct Bestellung {
    // "pub" ist Erweiterung aus Kapitel 6, "Collections und Slices"
    pub anzahl: i32,
    pub sorte: String,
    pub original: String,
}

// Rumpfimplementierung
// impl FromStr for Bestellung {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         todo!()
//     }
// }

impl FromStr for Bestellung {
    type Err = String;

    fn from_str(s: &str) -> Result<Bestellung, Self::Err> {
        let mut split_iterator = s.split("x");

        if split_iterator.clone().count() != 2 {
            return Err(
                "Kann Bestellung nicht verarbeiten".to_string()
            );
        }

        let erkannte_anzahl = split_iterator
            .next()
            .unwrap_or("0")
            .trim_end()
            .parse()
            .unwrap();

        let erkannte_sorte: String = split_iterator
            .next()
            .unwrap_or("")
            .trim_start()
            .parse()
            .unwrap();

        let erkannte_sorte =
            erkannte_sorte.replace("77", "Pizza Salami");

        let bestellung = Bestellung {
            anzahl: erkannte_anzahl,
            sorte: erkannte_sorte,
            original: s.to_string(),
        };

        Ok(bestellung)
    }
}