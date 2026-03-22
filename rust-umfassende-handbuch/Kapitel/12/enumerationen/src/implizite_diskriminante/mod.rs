#![allow(unused_variables)]
#![allow(dead_code)]

enum Farben {
    Rot,
    Gruen,
    Blau,
}

pub fn main() {
    {
        // Der Compiler prüft, ob alle Fälle abgedeckt sind
        fn erhalte_farbe(farbe: Farben) {
            // match farbe {
            //     Farben::Rot => {}
            //     Farben::Blau => {}
            // }
        }
    }

    {
        // Enumerationen ohne Varianten
        enum Leer {}

        // let fang_die_panic: Leer = panic!();

        let kein_wert: Option<Leer> = None;
        println!("{}", std::mem::size_of_val(&kein_wert)); // 0

        fn unfehlbare_operation() -> Result<i32, Leer> {
            // ...
            Ok(42)
        }

        fn sicherheitszone() -> Option<i32> {
            let ergebnis = unfehlbare_operation();
            println!(
                "Größe von Fehler: {}",
                std::mem::size_of_val(&ergebnis) // 4
            );

            match ergebnis {
                Ok(wert) => Some(wert),
                Err(_) => None,
            }
        }

        sicherheitszone();
    }

    {
        struct Rot;

        use Farben::*;

        // Achtung, Unit-ähnlicher Typ Rot!
        Rot;
        Blau; // Variante von Farben
        Gruen; // Variante von Farben

        // Hier wird die Struktur Rot,
        // nicht die Variante Rot zugewiesen!
        let r = Rot;

        {
            // Absicherung in lokalem Block
            use Farben::*;

            Rot; // Variante von Farben
            Blau; // Variante von Farben
            Gruen; // Variante von Farben

            let r = Rot; // Variante von Farben
        }
    }

    {
        // Zuweisung und Neuzuweisung
        let mut farbe: Farben = Farben::Rot;
        farbe = Farben::Blau;
    }

    let rot = Farben::Rot;
    let rot_diskriminante = rot as isize; // 0
    println!("{rot_diskriminante}");

    let gruen = Farben::Gruen;
    let gruen_diskriminante = gruen as isize; // 1
    println!("{gruen_diskriminante}");

    let blau = Farben::Blau;
    let blau_diskriminante = blau as isize; // 2
    println!("{blau_diskriminante}");

    {
        // Bereits herausbewegt
        let rot = Farben::Rot;
        // Ok
        let rot2 = rot;
        // Fehler, rot wurde schon zu rot2 verschoben
        // let rot3 = rot;
    }

    {
        // Bereits herausbewegt
        let rot: &Farben = &Farben::Rot;
        // Ok, geteilte Referenz
        let rot2 = rot;
        // Ok, geteilte Referenz
        let rot3 = rot;

        let blau: &mut Farben = &mut Farben::Blau;
        // ...
    }
}
