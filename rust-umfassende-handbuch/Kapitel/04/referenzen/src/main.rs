use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ops::Deref;

fn main() {
    {
        // Adressen, Zeiger und Referenzen
        let zahl: i32 = 6;
        // Die Referenz zeigt auf die Adresse von "zahl"
        let referenz_auf_zahl: &i32 = &zahl;

        // Referenzen sind Copy
        let weitere_referenz: &i32 = referenz_auf_zahl;
        let summe: i32 = zahl + referenz_auf_zahl + weitere_referenz;
        // Summe: 18

        // Die Speicheradressen sind identisch
        println!(
            "Variable: {:p}, Referenz 1: {:p}, Referenz 2: {:p}",
            &zahl, referenz_auf_zahl, weitere_referenz
        );
        // Alle geben die gleiche Adresse aus, z. B. 0xe4d7b9f85c

        {
            // Thema Dereferenzierung von Referenzen

            // Automatische Dereferenzierung
            {
                let zahl = 12;
                let referenz = &zahl;

                //println!("Die Adresse: {:p}", *referenz); // Fehler!
                println!("Die Adresse von Referenz: {:p}", referenz); // Richtig -> 0x93cdbdf39c
                println!("Der Wert hinter 'referenz' ist: {}", *referenz); // Richtig -> 12
                println!("(Auto) Der Wert hinter 'referenz' ist: {}", referenz); // Richtig -> 12
            }


            // Die Adressen der Referenzen miteinander vergleichen
            let zahl = 12;
            println!("Sind gleich: {}", &12 == &zahl); // true

            {
                // Zunächst "manuell"
                let ptr_1 = &12 as *const i32;

                let zahl = 12;
                let ptr_2 = &zahl as *const i32;
                println!(
                    "Zeiger ptr_1 ({:p}) und ptr_2 ({:p}) sind nicht gleich: {}",
                    ptr_1,
                    ptr_2,
                    ptr_1 != ptr_2
                ); // Richtig! -> true
            }
            {
                // ... dann automatisch
                println!(
                    "Haben gleiche Adressen: {}",
                    std::ptr::eq(&12, &zahl)
                ); // Richtig! -> false
            }
        }
    }

    {
        // Geteilte Referenzen
        let a = 4;
        let b = 6;

        {
            // let ergebnis =
            //     {
            //         let sum = a + b;
            //         // Die Referenz auf "sum" zurückgeben
            //         &sum // Fehler!
            //     };
            // println!("Das Ergebnis ist: {ergebnis}");
        }

        {
            let ergebnis =
                {
                    let sum = a + b;
                    let referenz_auf_heap = Box::new(sum);
                    referenz_auf_heap
                };

            // mit * wird die Box dereferenziert
            let referenz_auf_summe = *ergebnis;
            println!("Die Summe vom Heap: {referenz_auf_summe}")
        }
    }

    {
        // Exklusive Referenzen
        {
            // Gültig, referenz lebt nur bis zahl neu gesetzt wird
            let mut zahl = 5;
            let referenz = &mut zahl;

            *referenz = 3;
            zahl = 2;
        }

        {
            let mut zahl = 5;
            let referenz = &mut zahl;
            let weitere_referenz = &mut zahl;

            // Fehler! „referenz“ und „weitere_referenz“ gleichzeitig
            // *referenz = 2; // einkommentieren für Fehler
            *weitere_referenz = 4;
        }

        {
            // Ungültig: Lebenslinien überschneiden sich
            {
                // Fehler Szenario 1
                let mut zahl = 5;
                let referenz = &mut zahl;

                // zahl = 2;
                // Fehler! Hierdurch lebt „referenz“ noch
                *referenz = 3;
            }

            {
                // Fehler Szenario 2
                let mut zahl = 5;
                let referenz = &mut zahl;

                // OK
                *referenz = 3;
                // zahl = 2;
                // Fehler! Hierdurch lebt „referenz“ noch
                *referenz = 4;
            }

            {
                let mut zahlen = vec![3, 5, 8];
                let letzte_zahl = &zahlen[1];

                // Fehler, ein (unscheinbarer) möglicher Dangling Pointer
                // zahlen.pop();

                // Referenz letzte_zahl lebt bis hier
                println!("{letzte_zahl}");
            }
        }

        {
            // &mut T zu *mut T
            let mut zahl = 12;

            let referenz = &mut zahl;
            // Zeiger manuell aus Referenz bilden
            let zeiger = referenz as *mut i32;

            // ... oder automatisch über das std::ptr Modul
            let zeiger = std::ptr::addr_of_mut!(zahl);

            println!("Vor dem Schreiben über Zeiger: {}", zahl); // 12

            unsafe {
                *zeiger = 4;

                let referenz_aus_zeiger = &(*zeiger);
                println!("Nach dem Schreiben über Zeiger: {}", referenz_aus_zeiger); // 4
            }
        }

        {
            // Wechsel zwischen &mut T und &T
            let mut zahl = 5;
            let schreibend = &mut zahl;
            let zeiger: *mut i32 = schreibend;

            // OK: &mut i32 zu &i32
            let nur_lesend: &i32 = schreibend;
            // Fehler: &i32 zu &mut i32 nicht erlaubt
            // let wieder_schreibend: &mut i32 = nur_lesend;

            // OK: *mut i32 zu *const i32
            let konstanter_zeiger: *const i32 = zeiger;
            // Fehler: *const i32 zu *mut i32 nicht erlaubt
            // let zeiger_wieder_schreibend: *mut i32 = konstanter_zeiger;
        }
    }

    {
        // Box mit String Slice, mit Bug
        let erster_wert = "Ungeöffnet";
        let mut eine_box = Box::new(erster_wert);
        *eine_box = "Geöffnet";
        println!("{}", *eine_box); // Geöffnet
        println!(
            "Box als lokale Variable: {:p} \
            und der Wert im Heap: {:p}",
            eine_box,
            *eine_box
        );
        let referenz_in_box: &&str = eine_box.borrow();
        let der_string_in_der_box: &str = *referenz_in_box; // Bug
        *eine_box = "Geschlossen";

        println!("{}", der_string_in_der_box); // Geöffnet, Fehler: alter Text

        println!("{}", *eine_box) // Geschlossen, richtig!
    }

    {
        let vektor = vec![1, 2, 3];
        let vektor_in_box = Box::new(vektor);

        // Fehler: vektor schon per Move verschoben!
        // println!("{}", vektor.len())
    }

    {
        // Box mit String Slice, mit Bugfix
        let erster_wert = "Ungeöffnet";
        let mut eine_box = Box::new(erster_wert);
        *eine_box = "Geöffnet";
        println!("{}", *eine_box); // Geöffnet
        let referenz_in_box: &&str = eine_box.borrow();
        // *eine_box = "Geschlossen";

        println!("{}", *referenz_in_box); // Geöffnet, Fehler: alter Text

        println!("{}", *eine_box) // Geschlossen, richtig!
    }

    {
        // Box Drop
        {
            // Mit Fehler
            let box_a = Box::new('a');
            let referenz_box_b: &char;

            {
                let box_b = Box::new('b');
                println!("Box a: {} box b: {}", *box_a, *box_b);
                // Fehler, box_b lebt kürzer als referenz_box_b
                // referenz_box_b = box_b.borrow();
            }

            // Fehler: Lebenszeit von box_b zu kurz
            // println!("Box a: {} Box b: {}", *box_a, referenz_box_b);
        }

        {
            let mut box_b = Box::new('b');

            // ...
            // Fehler, drop darf nicht direkt aufgerufen werden
            // box_b.drop();

            // richtig:
            std::mem::drop(box_b);
        }

        {
            // Box lebt über Move weiter
            let box_a = Box::new('a');
            let referenz_box_b: &char;

            let box_b = {
                let box_b = Box::new('b');
                println!("Box a: {} Box b: {}", *box_a, *box_b);
                // Fehler, box_b lebt kürzer als referenz_box_b
                box_b
            };

            // Jetzt kann mit box_b gearbeitet werden
            referenz_box_b = box_b.borrow();
            // Fehler, box_b ist schon ungültig
            println!("Box a: {} Box b: {}", *box_a, *box_b);
            // Folgefehler wegen Lebenszeit von box_b
            println!("Box a: {} Box b: {}", *box_a, referenz_box_b.to_string());
        }
    }

    {
        // Gelöstes Listing 1.6
        let ergebnis =
            {
                // Wir speichern 65535 (u16::MAX) Datensätze
                let mut daten = vec![
                    // Befülle mit Wert
                    "Ein Datensatz";
                    // u16::MAX (65535) Mal
                    usize::from(u16::MAX)
                ];

                let size = std::mem::size_of_val(&*daten);
                println!("Anzahl Bytes des Datensatze: {size}");
                // Ausgabe: 1048560 auf einem 64-Bit System

                Box::new(daten)
            };

        let size_of_box = std::mem::size_of_val(&ergebnis);
        println!("Anzahl Bytes der Box: {size_of_box}");
        // Ausgabe: 8

        println!("Anzahl der Datensätze: {}", ergebnis.len());
    }
}