pub fn main() {
    {
        {
            let beliebige_zahl = 19;
        }

        {
            // Literal-Präfix
            let dezimal = 16;
            let oktal = 0o20;
            let hex = 0x10;
            let binaer = 0b10000;

            println!(
                " \
            Dez: {dezimal}, \
            Oktal: {oktal}, \
            Hex: {hex}, \
            Binär {binaer}"
            );
        }

        {
            // Literal-Suffix
            let dezimal = 16u64;
            let oktal = 0o20u32;
            let hex = 0x10u16;
            let binaer = 0b10000u8;

            println!(
                " \
            Dez: {dezimal}, \
            Oktal: {oktal}, \
            Hex: {hex}, \
            Binär {binaer}"
            );
        }

        {
            // Unterstriche in Literalen
            let dezimal = 16_u64;
            let oktal = 0o_20_u32;
            let hex = 0x_10_u16;
            let binaer = 0b_0001_0000_u8;

            let eine_milliarde = 1_000_000_000_i32;

            println!(
                " \
            Dez: {dezimal}, \
            Oktal: {oktal}, \
            Hex: {hex}, \
            Binär {binaer}"
            );
        }

        {
            // Verschiedene Integer-Typen

            // Typangabe in Definition
            let beliebige_zahl: i16 = 19;
            let kleine_zahl: i8 = 4;
            let positive_zahl: u32 = 1_000_000;

            // Bestimmt durch das Literal
            let sehr_große_zahl = 1_000_000_000_000_u128;
            let sehr_große_negative_zahl = -1_000_000_000_000_i128;

            // Maschinenwörter
            let a = isize::MIN;
            let b = usize::MAX;
        }

        {
            // Operatoren
            let erste_zahl = 19;
            let zweite_zahl = 23;

            let summe = erste_zahl + zweite_zahl;

            println!("Die Summe ist: {summe}");
        }

        {
            // Operator und Typinferenz
            let erste_zahl: u16 = 19;
            // Automatisch inferiert als u16
            let zweite_zahl = 23;

            // Ebenfalls u16
            let summe = erste_zahl + zweite_zahl;
        }



        // Integer Konstanten
        println!("{}, {}, {}", u32::MAX, u32::MIN, u32::BITS);

        {
            // Beispiel: Overflow beim Kompilieren erkennen
            let a: i8 = 100;
            // let b :i8 = 100;
            //
            // Rust kompiliert nicht weiter,
            // da ein Overflow erkennbar ist
            // let sum = a + b;
            // println!("{sum}");
        }

        {
            // Overflow zur Laufzeit
            let mut input = String::from("0");
            // Eine Zahl von der Konsole einlesen
            std::io::stdin().read_line(&mut input);
            // Zeileneinrückung am Ende entfernen
            let input = input.trim();

            // FromStr einbinden
            use std::str::FromStr;
            // Zu einer i8-Zahl konvertieren
            let input = i8::from_str(input).unwrap();

            let a: i8 = 100;
            // Kompiliert, aber Laufzeitfehler!
            let sum = a + input;
            println!("{sum}");
        }

        // checked_add overflow
        let wert_option = 100_i8.checked_add(100);

        // checked_add mit Ausdruck zerlegen
        let wert_option = 100_i8.checked_add(100);

        if let Some(zahl) = wert_option {
            println!("{}", zahl);
        } else {
            println!("Overflow")
        }

        {
            // Beispiel für Wrapping
            let wrapping_result = 100_i8.wrapping_add(100);
            println!("Wrapping: {wrapping_result}"); // -56
        }

        {
            // Beispiel für Saturating
            let saturating_result = 100_i8.saturating_add(100);
            println!("Saturating: {saturating_result}"); // 127
        }

        {
            // Beispiel für Overflowing
            let overflowing_result = 100_i8.overflowing_add(100);
            println!(
                "Overflowing: Wert = {}, ist übergelaufen = {}",
                overflowing_result.0, overflowing_result.1
            );
            // Overflowing: Wert = -56, ist übergelaufen = true
        }
    }
}