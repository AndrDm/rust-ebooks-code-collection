// Steps
// -> struct Speise einführen
// -> sorte: String zu sorte: Speise
// -> sorte: Speise zu speisen: Vec<Speise>
// -> original: String, entfernen
// -> anzahl: i32 in struct Speise überführen

#[derive(Debug)]
struct Bestellung {
    speisen: Vec<Speise>,
    original: String,
}

impl Bestellung {
    fn new(speisen: Vec<Speise>) -> Bestellung {
        Bestellung {
            speisen,
            original: "".to_string(),
        }
    }

    fn rechnungsbetrag(&self) -> Preis {
        let betrag = self.speisen
            // Von Speisen einen Iterator anfordern
            .iter()
            // Aus allen Speisen einen Wert bilden
            .fold(
                0.0,
                // Callback: Mit jeder Iteration den Preis
                // auf den Zwischenstand addieren.
                |gesamt, speise| {
                    let preis_aktuell =
                        // Achtung, Typumwandlung nötig, weil
                        // f64 und i32 nicht multipliziert
                        // werden können
                        speise.preis.0 * speise.anzahl as f64;

                    // An nächsten Durchgang weitergeben
                    gesamt + preis_aktuell
                },
            );

        Preis(betrag)
    }

    // fn rechnungsbetrag_trinkgeld(
    //     &self,
    //     trinkgeld: f64,
    //     kartenzahlung: bool,
    // ) -> f64 {
    //     // ...
    //     todo!();
    // }


    fn verwerfen(self) {
        // ... aufräumarbeiten im System durchführen
    }
}

impl Bestellung {
    fn foo(self: Box<Self>) {
        println!("In der Box!");
    }
}

impl Default for Bestellung {
    fn default() -> Self {
        Bestellung {
            speisen: Vec::new(),
            original: "".to_string(),
        }
    }
}

#[derive(Debug)]
struct Speise {
    anzahl: i32,
    sorte: String,
    preis: Preis,
}

#[derive(Debug)]
struct Preis(f64);

impl Speise {
    fn new(sorte: String, preis: Preis) -> Speise {
        Speise {
            anzahl: 1,
            sorte,
            preis,
        }
    }
}

pub fn main() {
    let bestellung = Bestellung::new(
        vec![
            Speise::new(
                "Pizza Salami".into(),
                Preis(22.0),
            ),
            Speise::new(
                "Pizzabrot".into(),
                Preis(3.2),
            ),
        ]
    );

    let zu_zahlen = bestellung.rechnungsbetrag();
    println!("Gesamtbetrag: {}", zu_zahlen.0); // 25.2

    // let bestellung = Bestellung {
    //     // ...
    // };
    //
    // bestellung.verwerfen();
    // // Fehler, "bestellung" schon verbraucht
    // println!("Rechnungsbetrag: {}", bestellung.rechnungsbetrag());
}