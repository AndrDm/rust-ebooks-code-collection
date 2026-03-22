fn main() {
    // Erste Pfade

    // Fehler, der Pfad muss angegeben werden
    // let map = HashMap::new();

    // Ok
    let map = std::collections::HashMap::<i32, String>::new();

    {
        // HashMap im lokalen Block bekanntmachen
        use std::collections::HashMap;

        // Ok, dank "use"
        let map = HashMap::<i32, String>::new();
    }

    {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let map = HashMap::<i32, String>::new();
        let set = HashSet::<String>::new();
    }

    {
        // Gleiche Pfade zusammenlegen
        use std::collections::{
            HashMap,
            HashSet,
        };

        let map = HashMap::<i32, String>::new();
        let set = HashSet::<String>::new();

        let v = std::collections::VecDeque::<i32>::new();
    }

    {
        // Alles mit * in Betracht ziehen
        use std::collections::*;

        let map = HashMap::<i32, String>::new();
        let set = HashSet::<String>::new();

        let v = VecDeque::<i32>::new();
    }

    {
        // Teilbäume
        use std::{
            collections::{
                HashMap,
                HashSet,
            },
            str::FromStr,
            fmt::Display,
        };

        // ...

        // Gegenüberstellung einzelner Pfade
        {
            use std::collections::HashMap;
            use std::collections::HashSet;
            use std::str::FromStr;
            use std::fmt::Display;

            // ...
        }

        {
            // Das Schlüsselwort self
            use std::any::{
                self, // für any::type_name
                Any, // für "type_id"
                TypeId, // für Typdeklaration :TypeId
            };

            let str = "Hallo, Rust";
            let type_id: TypeId = str.type_id();
            println!(
                "Typename: {} und Typ-ID: {:?}",
                any::type_name::<str>(),
                type_id
            );
        }
    }

    {
        // as
        use std::string::String as std_string;
        use std::collections as containers;

        let string = std_string::new();

        let map = containers::HashMap::<i32, std_string>::new();
    }

    {
        // as _

        // Fehler, from_str ist nicht bekannt
        use std::num::ParseIntError;

        // let zahl = i32::from_str("42");

        {
            use std::{
                num::ParseIntError,
                str::FromStr as _,
            };

            let zahl: Result<i32, ParseIntError> = i32::from_str("42");
        }
    }


    {
        // pub use
        {
            // pub use
            mod modul_a {
                // Ok, "schreibe_modul_c" ist nicht eingeschränkt
                pub use modul_b::modul_c::schreibe_modul_c;

                pub mod modul_b {
                    pub mod modul_c {
                        pub fn schreibe_modul_c() {}
                    }
                }
            }
        }

        {
            mod modul_a {
                // Fehler, schreibe_modul_c ist privat!
                // pub use modul_b::modul_c::schreibe_modul_c;

                pub mod modul_b {
                    pub mod modul_c {
                        fn schreibe_modul_c() {}
                    }
                }
            }
        }

        {
            mod modul_a {
                // Ok, auf Crate-Grenze
                pub(crate) use modul_b::modul_c::schreibe_modul_c;

                pub mod modul_b {
                    // super -> modul_b, super::super -> modul_b::modul_a
                    pub(in super::super) mod modul_c {
                        pub fn schreibe_modul_c() {}
                    }
                }
            }

            modul_a::schreibe_modul_c();
        }
    }
}