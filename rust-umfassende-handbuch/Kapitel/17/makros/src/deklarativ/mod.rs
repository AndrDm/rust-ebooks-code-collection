#![cfg(feature = "deklarativ")]

macro_rules! ausgeben {
    ($g:expr) => {
        println!("{}", $g);
        // let s: String = $g;
        // let s: String = $g;
        // $g.make_ascii_uppercase();
    };
    (H R:) => {
        println!("Hallo, Rust");
    };
    ($g:expr, $t:ty) => {
        println!("{}, Typ: {}", $g, core::any::type_name::<$t>());
    };
}

macro_rules! todo {
    ($prio:literal) => {
        println!("Noch zu erledigen, Priorität {}", $prio);
        // Auf Original verweisen
        core::todo!();
    };
}

pub fn main() {
    {
        // Makro ohne Argumente
        macro_rules! ausgeben {
            () => {
                let str = "Leer!";
                println!("{str}");
            };
        }

        ausgeben!(
            // Argumente
        );
        ausgeben! [
            // Argumente
        ];
        ausgeben! {
            // Argumente
        };
    }

    // Literale passen
    ausgeben!(H R:);

    ausgeben!("Hallo", i32);
    // Hallo, Typ: i32
    ausgeben!("Hallo", String);
    // Hallo, Typ: alloc::string::String

    {
        // Strukturen einführen, doppelte geschweifte Klammern

        macro_rules! neue_struktur {
            (
                $name:ident,
                $feld:ident,
                $feld_typ:ty,
                $feld_init:expr
            ) => {{
                #[derive(Debug)]
                struct $name {
                    $feld: $feld_typ,
                }

                impl $name {
                    pub fn new() -> $name {
                        $name { $feld: $feld_init }
                    }
                }

                $name::new()
            }};
        }

        let p = neue_struktur!(Person, name, String, "Freda".into());
        println!("Instanz von Person: {p:?}");
        // Instanz von Person: Person { name: "Freda" }
    }

    {
        // ident vs. ty
        macro_rules! neue_struktur {
            (
                $name:ident,
                $feld:ident,
                $feld_typ:path,
                $feld_init:expr
            ) => {{
                #[derive(Debug)]
                struct $name {
                    $feld: $feld_typ,
                }

                impl $name {
                    pub fn new() -> $name {
                        $name { $feld: $feld_init }
                    }
                }

                let v = Vec::<$feld_typ>::new();

                $name::new()
            }};
        }

        let p = neue_struktur!(Person, name, String, "Freda".into());
        // Ok, Pfad verweist auf Datentyp
        let p = neue_struktur!(Person, name, std::string::String, "Freda".into());
        // Fehler:
        // Ein Pfad, der allerdings keinen Datentyp sondern ein Modul adressiert
        // let p = neue_struktur!(Person, name, std::string, "Freda".into());
    }

    {
        // Strukturen einführen, auch in den äußeren Gültigkeitsbereich entlassen
        macro_rules! struktur {
            (
                $name:ident,
                $feld:ident,
                $feld_typ:ty,
                $feld_init:expr
            ) => {
                #[derive(Debug)]
                struct $name {
                    $feld: $feld_typ,
                }

                impl $name {
                    pub fn new() -> $name {
                        $name { $feld: $feld_init }
                    }
                }
            };
        }

        // Fehler, Person ist nicht bekannt
        // println!("Typ: {}", any::type_name::<Person>());
        struktur!(PersonNeu, name, String, "Fred".into());
        // Ok, Struktur wurde im Gültigkeitsbereich eingefügt
        let p = PersonNeu::new();
        println!("PersonNeu: {:?}", p);
        // PersonNeu: PersonNeu { name: "Fred" }
    }

    {
        // Move oder Referenz

        {
            // Move
            let str = String::from("Hallo, Rust");

            // ausgeben!(str);
            // Ok oder Fehler?
            // ausgeben!(str);
        }

        {
            // Veränderliche Referenz
            // let mut str = String::from("Hallo, Rust");
            // let str_ref = &str;
            // ausgeben!(str);
            // ausgeben!(str);
            // println!("Noch aktiv: {str_ref}");
        }
    }

    {
        // Typsicherheit
        ausgeben!("Hallo".to_string());
        // Fehler, nur String kompatibel
        ausgeben!(42_i32);
    }

    {
        // Funktionen
        macro_rules! funktion {
            ($f: expr) => {
                let a = $f;
                let b = &a;

                assert_eq!(a, *b);
            };
        }

        fn main() {
            let mut v = vec![1, 2, 3];

            // Ok
            funktion!(v[0]);

            // Fehler! Ruft pop zweimal auf
            funktion!(v.pop());
        }

        main();
    }

    {
        // Wiederholungs-Token
        macro_rules! ausgeben_ {
            ( $( $t:ty )*, $( $g:expr )*  ) => {
                $(
                    println!("Ausgabe: {}", $g );
                )*

                $(
                    println!("Typname: {}", core::any::type_name::<$t>());
                )*
                // ... mit $f arbeiten
            };
            // (  $( $g:expr )*  ) => {
            //     // $(
            //     //     println!("Ausgabe: {}", $g );
            //     // )+ // Fehler, Wiederholungs-Token stimmt nicht überein
            //
            //     // ... mit $i arbeiten
            // };

        }

        macro_rules! gemeinsam {
            ( $( $t:ty )*, $( $g:expr )*  ) => {
                $(
                    println!("Ausgabe: {}", $g );
                    println!("Typname: {}", core::any::type_name::<$t>());
                )*
            };
        }

        // Ok, beide Argumente fehlen
        ausgeben_!(,);

        ausgeben_!(i32 String, "Ein Ausdruck");

        // Ok, gleiche Anzahl der Argumente
        gemeinsam!(i32 String, "Zwei" "Ausdrücke");
        // Fehler, abweichende Anzahl der Argumente
        // gemeinsam!(String String, "Ein Ausdruck");

        // ausgeben_!("Hallo, Rust!");
        // ausgeben_!(2 "Hallo" "," "Rust" "!");
    }

    {
        // Shadowing

        // todo!(1);
    }
}
