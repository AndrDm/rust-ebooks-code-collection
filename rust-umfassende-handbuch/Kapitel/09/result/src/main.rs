
fn main() {
    {
        // Die Varianten von Result<T, E>
        let ok: Result<_, &str> = Ok(21);
        let fehler: Result<i32, _> = Err("Ein Fehler ist aufgetreten");

        // Gehören dem gleichen Datentyp an
        let mut ergebnis: Result<i32, &str> = ok;
        // ...
        ergebnis = fehler;
        // ...
    }

    {
        // Warnung, wenn Fehler ignoriert
        "42".parse::<i32>();
    }

    {
        // Parse Beispiele
        use std::num::ParseIntError;

        fn parse_zahl(str: &str) -> Result<i32, ParseIntError> {
            str.parse::<i32>()
        }

        let werte = [
            || parse_zahl("42"),
            || parse_zahl("abc"),
        ];

        for callback in werte {
            let ergebnis = callback();

            match ergebnis {
                Ok(zahl) => println!("Die Zahl ist {zahl}"),
                Err(fehler) => println!("Ein Fehler: {fehler}")
            }
        }
    }

    {
        // Die Funktionen ok und err
        let ergebnis = "42".parse::<i32>();

        let zahl: Option<i32> = ergebnis.ok();
        // Fehler, ergebnis ist schon verbraucht
        // let fehler: Option<ParseIntError> = ergebnis.err();
    }

    {
        // Referenzen in Result
        use std::num::ParseIntError;

        let ergebnis = "42.3".parse::<i32>();

        let ergebnis_ref = ergebnis.as_ref();
        let zahl: Option<&i32> = ergebnis_ref.ok();

        let fehler: Option<&ParseIntError> = ergebnis_ref.err();
    }

    {
        // Clone in Result
        use std::num::ParseIntError;

        let ergebnis = "42".parse::<i32>();

        let zahl: Option<i32> = ergebnis.clone().ok();
        // Ok, zahl wurde aus dem Klon gelesen
        let fehler: Option<ParseIntError> = ergebnis.err();
    }

    {
        // Referenzen und Deref
        fn lese_key_aus_datenbank() -> Result<Box<i32>, String> {
            // Auslesen...
            let key = 42;

            Ok(Box::new(key))
        }

        let ergebnis: Result<Box<i32>, String> = lese_key_aus_datenbank();

        // Die Box direkt dereferenzieren
        let schluessel_ergebnis: Result<&i32, &String> = ergebnis.as_deref();
        let schluessel: &i32 = schluessel_ergebnis.unwrap();
    }

    {
        // Result implementiert IntoIter
        let ergebnis = "42".parse::<i32>();
        // Durch "&" den Move vermeiden
        for zahl in &ergebnis {
            println!("Aus der Schleife: {zahl}"); // 42
        }

        // ...

        let ergebnis = "abc".parse::<i32>();
        // Durch "&" den Move vermeiden
        for zahl in &ergebnis {
            // Wird nicht durchlaufen, "ergebnis" ist Err(E)
            println!("Aus der Schleife: {zahl}");
        }

        // Werkzeugfunktionen erreichbar

        let iterator = "65".parse::<u32>().into_iter();
        let character = iterator.map(|result| {
            // In einen Buchstaben umwandeln
            char::from_u32(result)
        });

        for buchstabe in character {
            // Mit Shadowing das Original überdecken
            if let Some(buchstabe) = buchstabe {
                println!("Buchstabe: {}", buchstabe);
            }
        } // Buchstabe: A
    }

    {
        // Die Funktionen is_ok und is_err
        let ergebnis = "42".parse::<i32>();

        let ist_zahl = ergebnis.is_ok(); // true
        let ist_fehler = ergebnis.is_err(); // false
    }

    {
        // unwrap und expect
        let ergebnis = "42".parse::<i32>();

        let zahl: i32 = ergebnis.clone().unwrap(); // Ok
        // let fehler: ParseIntError = ergebnis.unwrap_err(); // Hier ein Fehler


        // besser: mit expect eine Nachricht hinterlassen
        let ergebnis = "abc".parse::<i32>();

        // let zahl: i32 = ergebnis.expect(
        //     "Der Wert 42 wird immer passen."
        // );
        // ...
        // thread 'main' panicked at 'Ein statischer Wert, wird immer passen.: ParseIntError { kind: InvalidDigit }'
    }

    {
        // unwrap_or und unwrap_or_else
        fn hole_favorit_aus_datenbank(sofort: bool) -> i32 {
            // Hier erfolgt ein kostspieliger Datenbankzugriff...
            println!("Aus der Datenbank ausgelesen. Sofort: {sofort}");
            42
        }

        let ergebnis = "42".parse::<i32>();

        // Führt die Anfage sofort aus, obwohl das Parsen erfolgreich war
        ergebnis.clone().unwrap_or(hole_favorit_aus_datenbank(true));
        // Ausgabe: Aus der Datenbank ausgelesen. Sofort: true

        // Führt nur aus, wenn ein Fehler auftrat
        ergebnis.unwrap_or_else(|fehler| hole_favorit_aus_datenbank(false));
        // Keine Ausgabe
    }

    {
        // unwrap_or_default
        let ergebnis = "abc".parse::<i32>();

        let zahl = ergebnis.unwrap_or_default(); // 0
        println!("unwrap_or_default: {zahl}");
    }

    {
        // Die Funktion and

        // Kombination 1
        let eingabe_1 = "123".parse::<i32>(); // Ok
        let eingabe_2 = "456".parse::<i32>(); // Ok

        if let Ok(ergebnis_2) = eingabe_1.and(eingabe_2) {
            println!("Beide Eingaben sind gültig: {ergebnis_2}");
        }

        // Kombination 2
        let eingabe_1 = "123".parse::<i32>(); // Ok
        let eingabe_2 = "def".parse::<i32>(); // Fehler
        if let Err(ergebnis_2) = eingabe_1.and(eingabe_2) {
            println!("Eingabe 2 ist ungültig: {ergebnis_2}");
        }

        // Kombination 3
        let eingabe_1 = "abc".parse::<i32>(); // Fehler
        let eingabe_2 = "456".parse::<i32>(); // Ok

        if let Err(fehler_1) = eingabe_1.and(eingabe_2) {
            println!("Eingabe 1 ist ungültig: {fehler_1}");
        }
    }

    {
        // Die Funktion or

        // Kombination 1
        let eingabe_1 = "123".parse::<i32>();
        let eingabe_2 = "abc".parse::<i32>();

        if let Ok(ergebnis_1) = eingabe_1.or(eingabe_2) {
            println!("Eine der Eingaben ist gültig: {ergebnis_1}");
        }
        // Eine der Eingaben ist gültig: 123

        // Kombination 2
        let eingabe_1 = "123".parse::<i32>();
        let eingabe_2 = "456".parse::<i32>();

        if let Ok(ergebnis_1) = eingabe_1.or(eingabe_2) {
            println!("Beide Eingaben sind gültig: {ergebnis_1}");
        }
        // Beide Eingaben sind gültig: 123

        // Kombination 3
        let eingabe_1 = "abc".parse::<i32>();
        let eingabe_2 = "456".parse::<i32>();

        if let Ok(ergebnis_2) = eingabe_1.or(eingabe_2) {
            println!("Eine der Eingaben ist gültig: {ergebnis_2}");
        }
        // Eine der Eingaben ist gültig: 456

        // Kombination 4
        let eingabe_1 = "abc".parse::<i32>();
        let eingabe_2 = "def".parse::<i32>();
        if let Err(ergebnis_2) = eingabe_1.or(eingabe_2) {
            println!("Beide Eingaben sind ungültig: {ergebnis_2}");
        }
        // Beide Eingaben sind ungültig: invalid digit found in string
    }

    {
        // Fragezeichen-Operator
        {
            // Problemfall mit verschiedenen Fehlerstrukturen
            use std::num::ParseIntError;

            fn lese_feature_datenbank() -> Result<bool, String> {
                // Hier findet der Zugriff auf die Datenbank statt
                let ist_feature_aktiv = true;
                Ok(ist_feature_aktiv)
            }

            fn parse_eingaben() -> Result<i32, ParseIntError> {
                let eingabe_1: i32 = "123".parse::<i32>()?;
                let eingabe_2: i32 = "456".parse::<i32>()?;

                // Fehler, hier wird jetzt auch ein
                // String-Fehler verarbeitet
                // if lese_feature_datenbank()? {
                //     Ok(eingabe_1 + eingabe_2)
                // } else {
                //     Ok(0)
                // }

                // Nur zum Kompilieren
                Ok(0)
            }

            let ergebnis: Result<i32, ParseIntError> = parse_eingaben();
        }

        {
            use std::error::Error;

            // Mit Error Trait
            fn lese_feature_datenbank() -> Result<bool, Box<dyn Error>> {
                // Hier findet der Zugriff auf die Datenbank statt
                let ist_feature_aktiv = true;
                Ok(ist_feature_aktiv)
            }

            fn parse_eingaben() -> Result<i32, Box<dyn Error>> {
                let eingabe_1: i32 = "123".parse::<i32>()?;
                let eingabe_2: i32 = "456".parse::<i32>()?;

                if lese_feature_datenbank()? {
                    Ok(eingabe_1 + eingabe_2)
                } else {
                    Ok(0)
                }
            }

            let ergebnis: Result<i32, Box<dyn Error>> = parse_eingaben();
        }
    }

    {
        use std::error::Error;
        use std::num::ParseIntError;

        fn lese_feature_datenbank() -> Result<bool, Box<dyn Error>> {
            // Hier findet der Zugriff auf die Datenbank statt

            let ist_feature_aktiv = true;
            Ok(ist_feature_aktiv)
        }

        // Downcasting
        fn parse_eingaben() -> Result<i32, Box<dyn Error>> {
            let eingabe_1: i32 = "abc".parse::<i32>()?;
            let eingabe_2: i32 = "456".parse::<i32>()?;
            if lese_feature_datenbank()? {
                Ok(eingabe_1 + eingabe_2)
            } else {
                Ok(0)
            }
        }
        let ergebnis: Result<i32, Box<dyn Error>> = parse_eingaben();

        {
            let ergebnis = ergebnis.as_ref();

            let fehler = ergebnis.unwrap_err();
            let ist_parse_error = fehler.is::<ParseIntError>(); // true
            println!("Ist ParseIntError: {ist_parse_error}");

            let parse_error: Option<&ParseIntError> = fehler.downcast_ref::<ParseIntError>();
            let parse_error = parse_error.unwrap();

            // Veränderliche Variante
            let mut ergebnis: Result<i32, Box<dyn Error>> = parse_eingaben();

            let ergebnis = ergebnis.as_mut();
            let fehler = ergebnis.unwrap_err();
            let parse_error: Option<&mut ParseIntError> = fehler.downcast_mut::<ParseIntError>();
            let parse_error = parse_error.unwrap();
            println!("ParseIntError: {parse_error:?}");
            // ParseIntError: Some(ParseIntError { kind: InvalidDigit })
        }

        {
            // Den Fehler mit Mustern erreichen

            let ergebnis = ergebnis.as_ref();

            if let Err(ergebnis) = ergebnis {
                if let Some(parse_error) = ergebnis.downcast_ref::<ParseIntError>() {
                    println!("Mit Muster zerlegt: {parse_error}");
                }
            }
        }

        {
            // Datenbankfehler mit eigener Fehlerstruktur

            #[derive(Debug)]
            struct DatenbankFehler {
                typ: DatenbankFehlerTyp,
            }

            #[derive(Debug)]
            enum DatenbankFehlerTyp {
                NichtVerbunden
            }

            impl DatenbankFehler {
                fn new_nicht_verbunden() -> DatenbankFehler {
                    DatenbankFehler {
                        typ: DatenbankFehlerTyp::NichtVerbunden
                    }
                }
            }

            use std::fmt::{Formatter, Display};

            // Nicht mit derive kompatibel, daher eigene Implementierung
            impl Display for DatenbankFehler {
                fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                    match self.typ {
                        DatenbankFehlerTyp::NichtVerbunden =>
                            f.write_str("Datenbank nicht verbunden"),
                        _ => f.write_str("Unbekannter Fehler")
                    }
                }
            }

            impl Error for DatenbankFehler {}

            fn lese_feature_datenbank() -> Result<bool, Box<dyn Error>> {
                // Angenommen, die Information kommt von der Datenbank-API
                let datenbank_verbunden = false;

                if !datenbank_verbunden {
                    // Fehler melden
                    return Err(
                        Box::new(
                            DatenbankFehler::new_nicht_verbunden()
                        )
                    );
                }

                // Hier findet der Zugriff auf die Datenbank statt
                let ist_feature_aktiv = true;
                Ok(ist_feature_aktiv)
            }

            fn parse_eingaben() -> Result<i32, Box<dyn Error>> {
                let eingabe_1: i32 = "123".parse::<i32>()?;
                let eingabe_2: i32 = "456".parse::<i32>()?;

                if lese_feature_datenbank()? {
                    Ok(eingabe_1 + eingabe_2)
                } else {
                    Ok(0)
                }
            }

            let ergebnis = parse_eingaben();
            // Fehler auslesen
            let fehler = ergebnis.unwrap_err();

            let fehler = fehler
                .downcast_ref::<DatenbankFehler>();

            // Fehler aus Option<&T> herausholen
            let fehler = fehler.unwrap();

            println!("Fehler beim Parsen: {fehler}");
            // Fehler beim Parsen: Datenbank nicht verbunden
        }
    }
}