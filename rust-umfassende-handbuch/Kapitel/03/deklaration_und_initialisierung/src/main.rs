#![allow(unused_variables, unused_assignments, dead_code)]

mod shadowing;

fn main() {
    {
        // Deklaration
        let alter: i8;

        // Deklaration und Initialisierung
        let alter: i8 = 42;

        // Fehler, passt nicht in den Wertebereich
        // let alter: i8 = 420;
    }

    {
        // Unveränderlichkeit

        let alter: i8 = 42;
        // Fehler
        // alter = 21;

        let mut alter: i8 = 42;

        // Ok
        alter = 21;


        {
            // In Kombination mit Shadowing
            let mut alter: i8 = 42;
            alter = 21;

            {
                // Ab hier sperren (Shadowing)
                let alter = alter;

                // Fehler, in diesem Bereich gesperrt
                // alter = 2;
            }

            {
                let alter_ref: &i8 = &alter;         // <-- Referenz leihen
                //         |
                // Referenz ist nicht veränderlich   //         |
                // alter_ref = 21;                   //         |
                //         |
                // Fehler                            //         |
                // alter = 21;                       //         |
                println!("{alter_ref}");             // <-- Referenz hier aktiv
                //         |
            }                                        // <-- Referenz zurückgeben
        }
    }

    {
        // Typinferenz

        let mut alter = 42; // inferiert zu i32
        alter = 21;
    }

    {
        // Statische Variablen
        static ANZAHL_BITS: u32 = u32::BITS;



        // Mit Literal (ebenfalls eine Konstante)
        static ALTER: i32 = 42;

        {
            // Konstante Funktionen am Beispiel von String

            // Ok, String::new ist konstant
            static mut STR: String = String::new();
            // Fehler, String::from ist nicht konstant
            // static STR2: String = String::from("Hallo");

            unsafe {
                STR = String::from("Hallo Rust");
                println!("Statischer String: {STR}");
            }

            // Auf einen Null-Zeiger initialisieren
            static mut ZEIGER: *mut String = std::ptr::null_mut();
            unsafe {
                // Achtung, lokale Variable
                let mut str = String::from("Hallo Rust");
                ZEIGER = std::ptr::addr_of_mut!(str);
                println!("Zeiger-String: {str}");
                // Zeiger-String: Hallo Rust
            } // <- Der Stack gibt "str" hier frei!

            // ...
            unsafe {
                // Einen neuen String mit dem Zeiger wiederherstellen
                // let str = String::from_raw_parts(
                //     ZEIGER as *mut u8,
                //     10, // Länge
                //     10  // Kapazität
                // );

                // Achtung: Zeigt auf freigegebenen Speicher!
                // println!("Zeiger-String: {str}");
                // Zeiger-String: P��`
            }
        }

        {
            // statische lokale Variable als Zähler
            fn zaehler() {
                static mut ZAEHLSTAND: u32 = 0;

                unsafe {
                    println!("Stand: {ZAEHLSTAND}");
                    ZAEHLSTAND += 1;
                }
            }

            zaehler(); // Stand: 0
            zaehler(); // Stand: 1
            zaehler(); // Stand: 2
            zaehler(); // Stand: 3
            zaehler(); // Stand: 4
        }
    }

    {
        // Shadowing
        shadowing::main();
    }
}
