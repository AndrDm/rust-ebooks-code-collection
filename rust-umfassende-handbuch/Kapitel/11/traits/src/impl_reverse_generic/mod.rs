pub fn main() {
    {
        // impl Trait: Abstrakte Rückgabetypen
        {
            // Lösung mit Box und dynamic dispatch
            fn gib_callback() -> Box<dyn Fn(i32, i32) -> i32> {
                // ...
                Box::new(|a, b| a + b)
            }

            let callback = gib_callback();

            // Der Compiler dereferenziert die Box automatisch
            let ergebnis = callback(2, 5); // 7

            println!("Ergebnis: {ergebnis}");
        }


        {
            // Lösung mit impl Trait als Rückgabetyp
            fn gib_callback() -> impl Fn(i32, i32) -> i32 {
                // ...

                // Die anonyme Funktion kann unmittelbar
                // auf dem Stack bewegt werden
                |a, b| a + b
            }

            let callback = gib_callback();

            // Keine Dereferenzierung, "callback" liegt auf dem Stack!
            let ergebnis = callback(2, 5); // 7

            println!("Ergebnis: {ergebnis}");
        }
    }

    {
        // Anonyme Typparameter
        {
            // impl Trait
            fn nimm_callback(f: impl Fn(i32, i32) -> i32) {
                let ergebnis = f(3, 5); // 15
                // ...
                println!("nimm_callback Ergebnis: {ergebnis}");
            }

            nimm_callback(|a, b| a * b);
        }

        {
            // Generischer Typparameter
            fn nimm_callback<F: Fn(i32, i32) -> i32>(f: F) {
                let ergebnis = f(3, 5); // 15
                // ...
                println!("nimm_callback Ergebnis: {ergebnis}");
            }

            nimm_callback(|a, b| a * b);
        }

        {
            // impl Trait und generischen Parameter verbinden

            // Zwei weitere Typparameter T und U
            fn nimm_callback<
                T: std::fmt::Display,
                U: std::fmt::Debug,
            >(
                f: impl Fn(i32, i32) -> i32,
                t: T,
                u: U,
            ) {
                let ergebnis = f(3, 5); // 15
                // ...
                println!("nimm_callback Ergebnis: {ergebnis}");

                println!("Argument T: {t}");
                println!("Argument U: {u:?}");
            }

            // Der Compiler inferiert alle Parameter
            nimm_callback(
                |a, b| a * b,
                "Hallo, Rust!".to_string(),
                'R',
            );

            // Der Compiler inferiert den anonymen Parameter,
            // alle anderen werden explizit angegeben
            nimm_callback::<String, char>(
                |a, b| a * b,
                "Hallo, Rust!".to_string(),
                'R',
            );
        }
    }
}