#![allow(dead_code)]
#![allow(unused)]

use std::marker::PhantomData;
use std::mem;

mod bestellsystem;
mod rekursion;
mod typ_alias;

fn main() {
    {
        // Unterschied Tupel und Struktur
        let person_1 = ("Alfred", 42);
        println!(
            "Name: {} und Alter: {}",
            // Zugriff über Index
            person_1.0,
            person_1.1,
        );

        struct Person {
            name: &'static str,
            alter: i32,
        }

        {
            // Generika werden in Kapitel 12, "Generische Programmierung"
            // weiter behandelt
            struct Person<T> {
                name: T,
                alter: i32,
            }

            // Person<&str>
            let mit_string_slice = Person {
                name: "Rusty",
                alter: 22,
            };
            // Person<String>
            let mit_string = Person {
                name: "Rusty".to_string(),
                alter: 22,
            };
        }

        let person_2 = Person {
            name: "Frida",
            alter: 44,
        };
        println!(
            "Name: {} und Alter: {}",
            // Zugriff über Bezeichner
            person_2.name,
            person_2.alter,
        );
    }

    {
        let person_1 = ("Alfred", 42);
        let person_2 = (44, "Frida");
    }

    type PersonTupel = (&'static str, i32);

    let person_3: PersonTupel = ("Fred", 23);

    // Tupel können weiterhin ohne Alias eingeführt werden
    let anonym = ("Nobody", 35);
    // ... Später, in einem anderen Modul
    let person_4: PersonTupel = anonym;

    {
        // Der struct-Ausdruck
        #[derive(Debug)]
        struct Person {
            name: &'static str,
            alter: i32,
        }

        let person_2 = Person {
            name: "Frida",
            alter: 44,
        };

        {
            // Komplexere Ausdrücke im Feld-Ausdruck
            let zufallsalter = Person {
                name: if rand::random::<i32>() % 2 == 0 {
                    "Fred"
                } else {
                    "Freda"
                },
                alter: (rand::random::<u32>() % 100) as i32,
            };
            println!("{zufallsalter:?}");
        }

        {
            // Kurzschreibweise für Felder
            let name = "Merlin";
            let _ = Person {
                // Kurzschreibweise
                name,
                // Reguläre Schreibweise
                alter: 67,
            };
        }
    }

    {
        // Update Syntax
        struct Person {
            name: &'static str,
            alter: i32,
            groesse: f64,
            anrede: &'static str,
            hobbys: Vec<&'static str>,
        }

        let tim = Person {
            name: "Tim",
            alter: 23,
            groesse: 1.82,
            anrede: "Herr",
            hobbys: vec!["Schnelle Autos", "Kochen"],
        };

        let jerry = Person {
            name: "Jerry",
            hobbys: vec!["Lesen", "Motorräder"],
            ..tim // Achtung, Hier kein Komma erlaubt
        };
    }

    {
        struct Punkt {
            x: i32,
            y: i32,
        }

        let mut a = Punkt { x: 1, y: 3 };

        // Neuzuweisung
        a = Punkt { x: 2, y: 5 };

        // Den Wert auslesen (Wertausdruck)
        println!("{}", a.x); // 2
        println!("{}", a.y); // 5

        // Die Felder schreiben (Speicherausdruck)
        a.x = 1;
        a.y = 2;
    }

    {
        struct Werkzeuge {
            print: fn(&'static str),
            addierer: fn(i32, i32) -> i32,
            // ... usw.
        }

        let werkzeuge = Werkzeuge {
            print: |str| println!("{str}"),
            addierer: |a, b| a + b,
        };

        // Fehler
        // let ergebnis = werkzeuge.addierer(2, 3);
        // Fehler
        // let schreibe = werkzeuge.print("Hallo, Rust");

        // Ok, Vorrang ist durch Gruppen-Ausdruck
        // explizit geregelt
        // let ergebnis = (werkzeuge.addierer)(2, 3);
        // Ok
        let schreibe = (werkzeuge.print)("Hallo, Rust");
    }

    {
        // Deref Beispiel String -> str
        let string = String::from("Hallo, Rust");

        // is_ascii ist eine Methode vom str Typ
        let ist_asccii = string.is_ascii();
        // Manuelle Dereferenzierung:
        //  * -> str
        //  & -> &str
        let ist_asccii = (&*string).is_ascii();
        // ...
    }

    {
        // Beispiel für Getter-Funktion im Sinne der Datenkapselung

        // Die Struktur Punkt
        struct Punkt {
            x: i32,
            y: i32,
        }

        // Ein inhärenter Implementierungsblock von Punkt
        impl Punkt {
            // Getter-Methode für x
            pub fn x(&self) -> i32 {
                // Log-Aufruf
                self.x
            }

            // Getter-Methode für y
            pub fn y(&self) -> i32 {
                // Log-Aufruf
                self.y
            }
        }

        fn main() {
            let p = Punkt { x: 2, y: 4 };
            let x = p.x();
            let y = p.y();
            // ...
        }
    }

    {
        // Sichtbarkeit im Modul (nicht sichtbar)

        mod geometrie {
            struct Punkt {
                x: i32,
                y: i32,
            }

            // Ok, gleiches Modul
            fn neuer_punkt() -> Punkt {
                Punkt { x: 6, y: 1 }
            }
        }

        // Fehler, außerhalb des Moduls
        // let b = geometrie::Punkt { x: 1, y: 3 };
        // Fehler, außerhalb des Moduls
        // let c = geometrie::neuer_punkt();
    }

    {
        // Sichtbarkeit im Modul (sichtbar)
        mod geometrie {
            pub struct Punkt {
                x: i32,
                pub y: i32,
            }

            // Ok, gleiches Modul
            pub fn neuer_punkt() -> Punkt {
                Punkt { x: 6, y: 1 }
            }
        }

        // Ok
        let b = geometrie::neuer_punkt();

        // Ok
        let y = b.y;

        // Fehler, x nicht sichtbar
        // let x = b.x;

        // let punkt = geometrie::Punkt {
        //     // Fehler, nicht sichtbar
        //     x: 2,
        //     // Ok
        //     y: 2,
        // };
    }

    {
        // Keine neuen Instanzen, aber alle Felder sichtbar
        mod geometrie {
            use core::marker::PhantomData;

            pub struct Punkt {
                pub x: i32,
                pub y: i32,
                _versteckt: PhantomData<bool>,
            }

            impl Punkt {
                pub fn new(x: i32, y: i32) -> Self {
                    Self { x, y, _versteckt: PhantomData }
                }
            }

            // Ok, gleiches Modul
            pub fn neuer_punkt() -> Punkt {
                Punkt {
                    x: 6,
                    y: 1,
                    _versteckt: PhantomData,
                }
            }
        }

        {
            // Aufruf mit new
            fn main() {
                let p = geometrie::Punkt::new(1, 2);
                // ...
            }

        }

        let p = geometrie::neuer_punkt();

        // Ok, Zugriff nur auf x und y
        println!("Koordinaten von p: {}, {}", p.x, p.y);

        // Ok, durch das Rest-Muster kein
        // unmittelbarer Zugriff auf "versteckt"
        let geometrie::Punkt { x, y, .. } = p;
        println!("Koordinaten von p: {}, {}", x, y);

        // let p = geometrie::Punkt {
        //     x: 0,
        //     y: 0,
        //     // Fehler, das Feld ist nicht zugänglich!
        //     _versteckt: PhantomData,
        // };
    }

    {
        // Generische Lebenszeitparameter
        // struct Person {
        //     name: &str,
        //     alter: i32,
        // }

        static PERSON: Person = Person {
            name: "Fred",
            alter: 45,
        };

        {
            // Wenn der Compiler nicht auf die Lebenszeiten in Strukturen achten würde
            let p: Person;

            {
                // let name_str = String::from("Fred");
                // p = Person {
                //     name: &name_str, // Die Referenz lebt nicht lang genug
                //     alter: 45,
                // };
            } // <-- name_str wird hier freigegeben

            // println!("{p:?}"); // Die Instanz p ist aber noch in Benutzung!
        }

        {
            // Unbegrenzte Lebenszeiten
            {
                // Fehler, 'a ist nicht zugewiesen
                // struct Person<'a> {
                //     name: &'static str,
                //     alter: i32,
                // }
            }

            // {
            // Version, in der die zu kurze Lebenszeit nicht erkannt wird
            // #[derive(Debug)]
            // struct Person {
            //     name: *const u8,
            //     name_laenge: usize,
            //     alter: i32,
            // }
            //
            // impl Person {
            //     fn new(str: &str) -> Person {
            //         Person {
            //             name: str.as_ptr(),
            //             name_laenge: str.len(),
            //             alter: 24,
            //         }
            //     }
            // }
            // }

            use std::marker::PhantomData;
            #[derive(Debug)]
            struct Person<'a> {
                name: *const u8,
                name_laenge: usize,
                alter: i32,
                // Wenn dieses Feld fehlt, übersetzt der Compiler
                // das Programm nicht mehr. Die Lebenszeit wäre ungebunden und
                // daher ungültig.
                // Der Datentyp u8 hat keinerlei syntaktische Bedeutung. Jeder andere
                // Typ möglich.
                _lebenszeit_marker: PhantomData<&'a u8>,
            }
            impl<'a> Person<'a> {
                // Die Lebenszeit des Arguments kann dank 'a
                // vom Compiler geprüft werden.
                fn new(str: &'a str) -> Person {
                    Person {
                        name: str.as_ptr(),
                        name_laenge: str.len(),
                        alter: 24,
                        // PhantomData ist ein Unit-Typ.
                        // Erstellt eine Instanz von PhantomData<&'a u8>
                        _lebenszeit_marker: PhantomData,
                    }
                }
            }

            let str = String::from("Peter");

            // Der Compiler soll den String nicht automatisch freigeben,
            // während wir mit einem Zeiger darauf arbeiten.
            let str = mem::ManuallyDrop::new(str);

            let p = {
                // Der Block reduziert die Lebenszeit des Strings
                // let lokaler_str = String::from("Petra");

                // let str = mem::ManuallyDrop::new(lokaler_str);
                Person::new(&str)
            };

            println!("Person: {p:?}");

            // String::from_raw_parts ist als unsafe deklariert.
            // Daher muss ein unsafe-Block die Funktion umgeben
            unsafe {
                // Der String wird wieder zusammengesetzt
                let str_zusammengesetzt = String::from_raw_parts(
                    // Der Aufruf fordert einen *mut u8 Zeiger
                    p.name.cast_mut(), // Adresse des Strings
                    p.name_laenge,     // Länge
                    p.name_laenge,     // Kapazität
                );

                // Szenario ohne PhantomData:
                // Der Speicherbereich war ungültig!
                // Im Debug-Modus führt die Ausgabe zu ungültigen
                // Werten und damit zu zufälligen Zeichen
                println!("{str_zusammengesetzt}"); // String lebt lang genug: Peter
            }

            // Wir müssen den String manuell freigeben (siehe oben)
            drop(std::mem::ManuallyDrop::into_inner(str));
        }

        {
            // PhantomData und Typparameter

            struct Kilogramm;
            struct Pfund;

            struct Gewicht<T>(f64, PhantomData<T>);

            impl Gewicht<Kilogramm> {
                const EIN_KG_IN_PFUND: f64 = 2.2046226218;

                fn new(betrag: f64) -> Gewicht<Kilogramm> {
                    Gewicht::<Kilogramm>(betrag, PhantomData)
                }

                fn to_pfund(&self) -> Gewicht<Pfund> {
                    Gewicht::<Pfund>::new(self.0 * Self::EIN_KG_IN_PFUND)
                }
                // Weitere Kg-spezifische Funktionen ...
            }

            impl Gewicht<Pfund> {
                const EIN_PFUND_IN_KG: f64 = 0.45359237;

                fn new(betrag: f64) -> Gewicht<Pfund> {
                    Gewicht::<Pfund>(betrag, PhantomData)
                }

                fn to_kg(&self) -> Gewicht<Kilogramm> {
                    Gewicht::<Kilogramm>::new(self.0 * Self::EIN_PFUND_IN_KG)
                }
                // Weitere Pfund-spezifische Funktionen ...
            }

            let gewicht_in_kilogramm = Gewicht::<Kilogramm>::new(22.09);
            let gewicht_in_pfund = Gewicht::<Pfund>::new(48.25);

            fn lese_gewicht_in_kg(g: &Gewicht<Kilogramm>) {
                // ...
                println!("Gewicht in Kg: {}", g.0);
                println!("Gewicht in Pfund: {}", g.to_pfund().0);
            }

            fn lese_gewicht_in_pfund(g: &Gewicht<Pfund>) {
                // ...
                println!("Gewicht in Pfund: {}", g.0);
                println!("Gewicht in Kg: {}", g.to_kg().0);
            }

            // Ok
            lese_gewicht_in_kg(&gewicht_in_kilogramm);

            // Fehler, nur Kg
            // lese_gewicht_in_kg(&gewicht_in_pfund);

            // Ok
            lese_gewicht_in_pfund(&gewicht_in_pfund);

            // Fehler, nur Pfund
            // lese_gewicht_in_pfund(&gewicht_in_kilogramm);
        }

        #[derive(Debug)]
        struct Person<'a> {
            name: &'a str,
            alter: i32,
        }

        let mut p = Person { name: "", alter: 0 };

        {
            //let temp_string = "Theo".to_string();
            // Fehler, die Referenz lebt kürzer als die Instanz
            //p.name = &temp_string;
        }

        println!("{p:?}");

        use std::io;

        fn main() {
            println!("Bitte den Namen angeben: ");
            let mut name = String::new();
            let _ = io::stdin().read_line(&mut name);

            println!("Bitte Alter angeben: ");
            let mut alter = String::new();
            let _ = io::stdin().read_line(&mut alter);

            let p = Person {
                name: &name.trim(),
                alter: alter
                    .trim()
                    .parse::<i32>()
                    .unwrap_or(0),
            };

            // ...
            println!("Erfasste Person: {p:?}");
        }

        // main();
    }

    {
        // Generische Lebenszeitparameter reduzieren oder vermeiden
        {
            #[derive(Debug)]
            struct Person<'a> {
                name: &'a str,
                alter: i32,
            }
        }

        {
            #[derive(Debug)]
            struct Person<'a, 'n> {
                name: &'a str,
                alter: &'n i32,
            }
        }

        {
            use smartstring::alias::String;

            struct Person<'n> {
                name: String,
                alter: &'n i32,
            }

            {
                // Test, passt, weil kleiner 23 Byte
                let struktur = Person {
                    name: "Alfred".into(),
                    alter: &74,
                };

                let speicher_name = &*struktur.name;

                use std::ptr::addr_of;

                println!("Adresse Person: {:p}", addr_of!(struktur));
                println!("Adresse Name: {:p}", addr_of!(*speicher_name));
            }

            {
                // Test, passt nicht, weil größer 23 Byte
                let struktur = Person {
                    name: "Ein ungewöhnlich langer Name".into(),
                    alter: &74,
                };

                let speicher_name = &*struktur.name;

                use std::ptr::addr_of;

                println!("Adresse Person: {:p}", addr_of!(struktur));
                println!("Adresse Name: {:p}", addr_of!(*speicher_name));
            }
        }
    }

    {
        // struct mit DST-Feld wird selbst DST
        // Ok
        struct NummernGruppe {
            id: i32,
            // Ein Slice als DST
            s: [i32],
        }

        // Ok
        struct Addierer {
            // Ein Trait-Objekt als DST
            t: dyn std::fmt::Write,
        }

        // Fehler, nur das letzte Feld darf ein DST sein
        // struct Durcheinander {
        //     s: [i32],
        //     string: String,
        // }

        // Fehler, wie oben
        // struct DoppeltesDurcheinander {
        //     s: [i32],
        //     t: [i32],
        // }

        // Fehler
        // let instanz = Box::new(NummernGruppe {
        //     id: 0,
        //     s: *Vec::from_iter([1 as i32, 2, 3, 4]).as_slice(),
        // });

        // ...

        // let s = StringSliceWrapper {
        //     id: 0,
        //     s: *Vec::from_iter([1 as i32, 2, 3, 4]).as_slice(),
        // };
    }

    {
        // Tupel-Strukturen
        struct Punkt(i32, i32);

        // Variante 1
        let a = Punkt(2, 4);

        // Variante 2:
        // Wird von Variante 1 benutzt
        let b = Punkt { 0: 2, 1: 4 };
    }

    {
        // Das newtype-Muster
        struct Gewicht(f64);

        let messung_1 = Gewicht(77.3);
        let messung_2 = Gewicht(83.5);
        // ...

        let gesamtgewicht = Gewicht(messung_1.0 + messung_2.0);
        println!("Gesamtgewicht: {}", gesamtgewicht.0); // 160.8
        // ...

        {
            use std::ops::Deref;

            // Deref Vorschlag
            impl Deref for Gewicht {
                type Target = f64;

                fn deref(&self) -> &f64 {
                    &self.0
                }
            }

            let gesamtgewicht = Gewicht(*messung_1 + 25.0);
            println!("Gesamtgewicht: {}", *gesamtgewicht); // 102.3
        }
    }

    {
        // Unit-Type Struct

        #[derive(PartialEq)]
        struct Werkzeuge;

        // Auch diese Schreibweise ist möglich
        struct Updater {}

        let werkzeug_1 = Werkzeuge;
        let werkzeuge_2 = Werkzeuge {};

        let sind_gleich = werkzeug_1 == werkzeuge_2; // true
        println!("Sind gleich: {sind_gleich}");

        const WERKZEUG: Werkzeuge = Werkzeuge;

        const fn arbeite_mit_werkzeugen() {
            const WERKZEUG_LOKAL: Werkzeuge = WERKZEUG;
            // ...
        }
    }

    {
        // Muster
        struct Person {
            name: &'static str,
            alter: i32,
        }

        let p = Person {
            name: "Fred",
            alter: 25,
        };

        let Person { name: s, alter: n } = p;
        // Weiter mit Variablen "s" und "n" arbeiten

        // Kurzschreibweise auch hier möglich
        let Person { name, alter } = p;
        // Weiter mit Variablen "name" und "alter" arbeiten

        // Rest-Muster und Unterstrich-Muster
        let Person { name, .. } = p;
        let Person { name: _, alter } = p;

        {
            // Den Kontrollfluss steuern
            struct Person {
                name: &'static str,
                alter: i32,
                hobbys: Option<Vec<String>>,
            }

            let p = Person {
                name: "Freda",
                alter: 22,
                hobbys: None,
            };

            // Mit dem Struktur-Muster den
            // Kontrollfluss steuern
            if let Person {
                name: _,
                alter: _,
                hobbys: Some(v),
            } = p
            {

                // Nur verarbeiten, wenn Hobbies
                // eingetragen wurden ...
            }
        }

        {
            // Ein Beispiel zu Tupel-Strukturen
            struct Punkt(i32, i32);

            let a = Punkt(2, 4);

            // Variante 1 mit runden Klammern
            let Punkt(x, y) = a;
            // Mit "x" und "y" weiter...

            // Variante 2 mit Struktur-Muster
            // und Indizes
            let Punkt { 0: x, 1: y } = a;
        }

        {
            // Die Kurzform self und Instanzfilter
            #[derive(Debug)]
            struct SpeicherGruss;

            impl SpeicherGruss {
                fn von_ueberall(&self) {
                    println!("Hallo, Rust!");
                }

                fn aus_der_box(self: &Box<Self>) {
                    println!("Hallo, Rust aus der Box!");
                }
            }

            let speicher = SpeicherGruss;
            // Ok
            speicher.von_ueberall();

            // Fehler, Instanz muss in einer Box sein
            // speicher.aus_der_box();

            let in_box = Box::new(speicher);
            // Ok, da in Box
            in_box.aus_der_box();

            // Ok
            in_box.von_ueberall();

            println!("{in_box:?}");

            {
                // manuelle Dereferenzierung);
                (*in_box).von_ueberall();
            }
        }
    }

    {
        // Ein einfacher impl-Block
        #[derive(Debug)]
        struct Gewicht(f64);

        impl Gewicht {
            const EINHEIT: &'static str = "Kilogramm";
        }

        // Verwendung
        let g = Gewicht(85.3);
        println!("Das Gewicht beträgt {g:?} {}", Gewicht::EINHEIT);
    }

    {
        // Generische Lebenszeiten in impl-Blöcken
        struct Person<'a> {
            name: &'a str,
            alter: i32,
        }

        // Fehler, 'a ist unbekannt
        // impl Person<'a> {
        //     // ...
        // }

        {
            // Vergleich mit Vec<T>
            struct Person<'a, H> {
                name: &'a str,
                alter: i32,
                hobbys: Vec<H>,
            }
        }

        impl<'b> Person<'b> {
            // ...
        }

        {
            // Anonyme Lebenszeiten
            impl Person<'_> {
                // ...
            }
        }

        struct Alter(i32);

        impl<'alt> Alter {
            // Fehler, generische Lebenszeit fehlt
            // fn sortiere_alter(&self, i: &i32, j: &i32) -> Vec<&i32> {
            //     let mut sortiert = Vec::from([&self.0, i, j]);
            //     sortiert.sort();
            //     sortiert
            // }

            // Lösung mit Lebenszeit aus dem Implementierungsblock
            // fn sortiere_alter(&'alt self, i: &'alt i32, j: &'alt i32) -> Vec<&i32> {
            //     let mut sortiert = Vec::from([&self.0, i, j]);
            //     sortiert.sort();
            //     sortiert
            // }
        }
    }

    bestellsystem::v2::main();
    typ_alias::main();
    rekursion::main();
}
