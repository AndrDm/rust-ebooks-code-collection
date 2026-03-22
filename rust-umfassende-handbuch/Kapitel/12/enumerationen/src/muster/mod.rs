pub fn main() {
    {
        // Unit-ähnlich

        #[derive(Debug)]
        enum Farben {
            Rot,
            Gruen,
            Blau,
        }

        // Bezeichner-Muster
        let rot = Farben::Rot;

        // Unterstrich-Muster
        let _ = Farben::Blau;

        fn erhalte_farbe(f: Farben) {
            if let gefiltert @ (Farben::Rot | Farben::Blau) = f {
                // Rot oder Blau erhalten...
                println!("Erhalten: {gefiltert:?}");
                return;
            }
            println!("Farbe nicht unterstützt");
        }
        erhalte_farbe(Farben::Rot); // Erhalten: Rot
        erhalte_farbe(Farben::Blau); // Erhalten: Blau
        erhalte_farbe(Farben::Gruen); // Farbe nicht unterstützt
    }

    {
        // Tupel
        enum Gewicht {
            Kilo(f64, String),
            Pfund(f64, String),
            Fehlmessung {
                nutzer: String,
                datum: String,
                meldung: String,
            },
        }

        if let Gewicht::Kilo(g, _) = Gewicht::Kilo(42.0, "2024-02-03".into()) {
            // mit g als Kilogramm arbeiten, nicht am Datum interessiert
        };

        if let Gewicht::Pfund(g, datum) = Gewicht::Pfund(42.0, "2024-02-07".into()) {
            // mit g als Pfund arbeiten, datum benennt den Tag,
            // an dem gemessen wurde
        };

        // Alternative mit match
        fn behandle_gewicht(g: Gewicht) {
            match g {
                Gewicht::Kilo(g, _) if g == 43.2 => {
                    // nur wenn das Gewicht mit 43.2 gemessen wurde
                }
                Gewicht::Kilo(g, _) => {
                    // ...
                }
                Gewicht::Pfund(g, datum) => {
                    // ...
                }
                Gewicht::Fehlmessung {
                    nutzer,
                    datum,
                    meldung,
                } => {
                    // ...
                }
            }
        }
    }
}
