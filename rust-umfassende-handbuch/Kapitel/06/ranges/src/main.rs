// Nur auf Nightly
// #![feature(step_trait)]

fn main() {
    {
        // Nur nötig, weil wir den Typ unten deklarieren
        use std::ops::Range;

        let elementbereich: Range<i32> = 1..5;

        let collection = elementbereich.collect::<Vec<_>>();
        println!("{:?}", collection); // [1, 2, 3, 4]
    }

    {
        // Bereichsgrenzen sind konstante Ausdrücke
        const start: i32 = 0;
        const ende: i32 = 32;

        let elementbereich = (start..ende); // 0..32
        println!("{elementbereich:?}");

        const fn hole_start() -> i32 { 1 }
        const fn hole_endwert() -> i32 { 4 }

        let elementbereich = (hole_start()..hole_endwert()); // 1..4
        println!("{elementbereich:?}");
    }

    {
        let elementbereich = 'A'..'[';
        println!("{elementbereich:?}"); // 'A'..'['
        println!("{}", elementbereich.collect::<String>()); // ABCDEFGHIJKLMNOPQRSTUVWXYZ
    }

    {
        let slices = ["Hallo", "Rust"];
        // OK
        let string = String::from_iter(slices.into_iter());
        // OK, into_iter implizit
        let string = String::from_iter(slices);
        // OK
        let string = String::from_iter(('A'..'D'));

        // Fehler
        // let string = String::from_iter(("A".."D"));
    }

    {
        // Beispiel: Trait Step implementieren

        // Step erfordert Vergleich, Sortierung und Kopien
        #[derive(PartialEq, PartialOrd, Clone)]
        struct Person {
            alter: i32,
        }

        // use std::iter::Step;

        // impl Step for Person {
        //     fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        //         todo!()
        //     }
        //
        //     fn forward_checked(start: Self, count: usize) -> Option<Self> {
        //         todo!()
        //     }
        //
        //     fn backward_checked(start: Self, count: usize) -> Option<Self> {
        //         todo!()
        //     }
        // }

        let elementbereich = (Person { alter: 1 }..Person { alter: 82 });
        // Ein Iterator steht jetzt zur Verfügung
        // for person in elementbereich {
        //     // mit person arbeiten
        // }
    }

    {
        // Beispiele contains und is_empty
        let elementbereich = 'A'..'Z';
        println!("Beispiele contains");
        println!("{}", elementbereich.contains(&'Y')); // true
        println!("{}", elementbereich.contains(&'Z')); // false

        println!("Beispiele is_empty");
        let elementbereich = 'Z'..'A';
        println!("{}", elementbereich.is_empty()); // true
        let elementbereich = 'Z'..'Z';
        println!("{}", elementbereich.is_empty()); // true
        let elementbereich = 'Z'..'[';
        println!("{}", elementbereich.is_empty()); // false
        println!("{}", elementbereich.contains(&'Z')); // true
    }

    {
        // Ranges per Move oder Clone bewegen
        {
            let bereich = 1..5;
            let neuer_besitzer = bereich;

            // Fehler, schon verschoben
            // println!("{bereich:?}");
        }

        {
            // Mit Clone arbeiten
            let bereich = 1..5;
            let neuer_besitzer = bereich.clone();

            // OK
            println!("{bereich:?}");
        }
    }

    {
        // Verschiedene Elementbereich Datentypen
        use std::ops::Range;

        let mut bereich: Range<i32>;

        // OK
        bereich = 1..5;
        // Fehler, nicht RangeFrom<i32>
        // bereich = 1..;
        // Fehler, nicht RangeFull
        // bereich = ..;
        // Fehler, nicht RangeTo<i32>
        // bereich = ..5;
        // ...

        // Lösen mit gemeinsamen Trait
        use std::ops::RangeBounds;
        use std::fmt::Debug;

        fn schreibe_bereich<T>(t: T)
            where T: RangeBounds<i32> + Debug {
            println!("{t:?}");
        }

        schreibe_bereich(..);
        schreibe_bereich(1..5);
        schreibe_bereich(1..);
        // ...

        #[derive(Debug)]
        struct RangeContainer<T>
            where T: RangeBounds<i32> + Debug {
            range: T,
        }

        let bereich_1 = RangeContainer { range: 1..5 };
        let bereich_2 = RangeContainer { range: 1.. };
        let bereich_3 = RangeContainer { range: .. };
        let bereich_4 = RangeContainer { range: ..5 };

        // Fehler, müssten einen gemeinsamen Typ haben
        // let array = [bereich_1, bereich_2, bereich_3, bereich_4];

        let debug_array: [&dyn Debug; 4] = [
            &bereich_1,
            &bereich_2,
            &bereich_3,
            &bereich_4
        ];
        for bereich in debug_array {
            // ...
            println!("{bereich:?}");
        }
    }

    {
        let x = 97_u32;
        match x {
            0..=31 => println!("Ein Steuerzeichen: 0x{x:X}"),
            32..=126 => println!("{}", char::from_u32(x).unwrap()),
            127 => println!("DEL"),
            _ => println!("Kein ASCII"),
        } // a
    }
}
