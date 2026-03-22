use std::io::Write;
use std::ops::Index;

fn main() {
    fn addiere(a: i32, b: i32) -> String {
        // lokale Variable
        let ergebnis = a + b;

        // Rückgabewert
        ergebnis.to_string()
    }

    {
        // Sichtbarkeit von Funktionen
        pub fn addiere(a: i32, b: i32) -> String {
            // ...
            todo!()
        }
    }

    {
        // Gültige und ungültige Funktionsbezeichner
        // Fehler
        // fn _() {}

        // OK, zwei Unterstriche hintereinander
        fn __() {}

        // Fehler, nur aus Zahlen
        // fn 123() {}

        // OK
        fn _123() {}

        // Fehler, ein Literal
        // fn 12_u8() {}

        // Fehler
        // fn 12_12() {}

        // OK
        fn a_12_12() {}

        // OK
        fn a_b() {}

        // Fehler, darf nicht mit Zahlen beginnen
        // fn 123a() {}

        // OK
        fn a123() {}

        // OK
        fn eine_überraschung () {}
    }

    // Rückgabetyp weglassen, wenn er () ist
    fn ausgabe(str: String) {
        println!("{str}");
    }

    {
        // voll ausgeschrieben
        fn ausgabe(str: String) -> () {
            println!("{str}");
        }
    }

    {
        // Early Return
        use std::num::ParseIntError;

        fn parse_zahl(str: String) {
            let moegliche_zahl: Result<i32, ParseIntError> = str.parse();
            if let Err(fehler) = moegliche_zahl {
                // Keine Zahl! Können nicht fortfahren
                println!("Fehlermeldung: {}", fehler.to_string());
                return;
            }

            let zahl = moegliche_zahl.unwrap();
            // mit der ausgelesenen Zahl weiterarbeiten ...
            println!("Zahl erhalten: {zahl}");
        }

        parse_zahl("42".to_string());
    }

    {
        // Eine Funktion an einer Variable zu weisen
        let addier_funktion: fn(i32, i32) -> String = addiere;
        let ergebnis = addier_funktion(1, 2); // "3"

        println!("Das Ergebnis der Addition: {ergebnis}");

        println!(
            "Größe eines Funktionszeigers: {} Byte",
            std::mem::size_of_val(&addiere)
        ); // 0 Bytes

        {
            let addier_funktion = addiere;
            let ergebnis = addier_funktion(1, 2); // "3"
        }
    }

    {
        // Funktionszeiger

        fn f1() {}
        fn f2() {}

        let mut eine_funktion = f1;
        // Fehler, f1 hat einen anderen Typ als f2!
        // eine_funktion = f2;

        {
            // Korrekt ist:
            let mut eine_funktion: fn() = f1;
            // OK
            eine_funktion = f2;
        }

        {
            // Beispiel mit Parametern
            fn fuege_zusammen(str1: &str, str2: &str) -> String {
                format!("{str1}{str2}")
            }

            let fun_zeiger: fn(&str, &str) -> String = fuege_zusammen;
            println!("{}", fun_zeiger("Hallo", ", Rust"));
        }
    }

    {
        fn callback(str: String) -> String {
            format!("Die Eingabe ist: {str}")
        }

        // Funktion als Parameter
        fn rufe_callback(argument: String, f: fn(String) -> String) {
            let ausgabe = f(argument);
            println!("{ausgabe}");
        }

        rufe_callback(
            "Hallo, Rust".into(),
            callback,
        );
        // Die Eingabe ist: Hallo, Rust
    }

    {
        // Referenzen und Lebenszeiten
        {
            fn ein_teil_string(str: &str) -> &str {
                if str.len() > 2 {
                    str.index(0..2)
                } else {
                    str
                }
            }

            // Statische Lebenszeit
            let ergebnis = ein_teil_string("Hallo"); // Ha
            println!("{ergebnis}");

            // lokale Variable, die bis zum Ende des Blocks lebt
            let string = "Rust".to_string();
            let ergebnis = ein_teil_string(&string); // Ru
            println!("{ergebnis}");
        }

        {
            // Fehler, der Compiler benötigt unsere Hilfe!
            // fn gib_laengere(s1: &str, s2: &str) -> &str {
            //     if s1.len() > s2.len() {
            //         s1
            //     } else {
            //         s2
            //     }
            // }

            fn gib_laengere<'a>(s1: &'a str, s2: &'a str) -> &'a str {
                if s1.len() > s2.len() {
                    s1
                } else {
                    s2
                }
            }
        }

        {
            // Lebenszeit auch ohne Parameter, wenn Referenz als Rückgabe
            {
                // Mit statischer Lebenszeit
                fn gruss() -> &'static str {
                    "Hallo Rust"
                }
            }

            {
                // Mit generischer Lebenszeit
                fn gruss<'a>() -> &'a str {
                    "Hallo Rust"
                }
            }
        }

        {
            // Varianz in Lebenszeiten

            fn gib_laengere<'a, 'b>(s1: &'a str, s2: &'a str) -> &'a str {
                if s1.len() > s2.len() {
                    s1
                } else {
                    s2
                }
            }

            let mut p: &'static str;
            let ergebnis_a_c: &str;
            {
                let a = "Die Lebenzeit a".to_string();
                // Lebenszeit 'a
                // <-- Referenzen auf a werden gültig
                {
                    let b = "Lebenszeit b".to_string();
                    // Lebenszeit 'b
                    // <-- Referenzen auf b werden gültig
                    {
                        // Lebenszeit 'c
                        // <-- Referenzen auf c werden gültig
                        let c = "Lebenszeit c".to_string();
                        static S: &str = "Statische Lebenszeit";
                        p = &S;

                        // OK
                        let ergebnis = gib_laengere(&a, &b);
                        // Zunächst OK, aber unten...
                        ergebnis_a_c = gib_laengere(&a, &c);
                        // OK
                        let ergebnis = gib_laengere(&a, S);
                        // <-- Referenzen auf c werden ungültig
                    }
                    // <-- Referenzen auf b werden ungültig
                }
                // Dieser Zugriff führt oben zum Fehler
                // println!("{}", ergebnis_a_c);

                // <-- Referenzen auf a werden ungültig
            }
            // S kann nicht mehr aufgelöst werden
            // Die Adresse ist aber gültig!
            println!("{}", p);
        }
    }

    {
        // const fn
        {
            const fn add(a: i32, b: i32) -> i32 {
                a + b
            }
            // Konstanter Kontext intakt: Vom Compiler berechnet
            const SUM: i32 = add(2, 5); // 7
            println!("{SUM}");

            {
                // Verlässt den konstanten Kontext:
                // Die Laufzeit führt "add" aus
                let sum: i32 = add(2, 5); // 7
                println!("{sum}");
            }
        }

        {
            const fn gib_groessere<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
                if *a > *b {
                    a
                } else {
                    b
                }
            }

            // Der Compiler wertet aus
            const ERGEBNIS: &i32 = gib_groessere(&3, &2);

            // ...
            // Zur Laufzeit
            let ergebnis = gib_groessere(&3, &2);

            // const fn foo(f: &mut dyn Write) {
            //     let _ = f.write(b"123");
            // }
        }

        {
            // const fn nicht kompatibel mit &mut Parametern
            // Fehler, &mut i32 ist nicht erlaubt
            // const fn add(a: &i32, b: &i32, ergebnis: &mut i32) {
            //     *ergebnis = a + b
            // }
        }

        {
            // const fn nicht kompatibel mit Funktionszeigern

            // Dient als Callback und gibt eine Ganzzahl zurück
            const fn zahl() -> i32 { 42 }

            // Soll den Callback aufrufen
            // Fehler, Funktionszeiger hier nicht erlaubt
            // const fn lese_zahl(callback: fn() -> i32) -> i32 {
            //     callback()
            // }

            // Fehler, siehe oben
            // const AUSGABE: i32 = lese_zahl(zahl);
        }

        {
            // Schleifen in const fn
            const fn zahlen() {
                const GRENZE: i32 = 4;
                let mut i = 0;

                // OK
                while i < GRENZE {
                    i += 1
                    // ...
                }

                i = 0;
                // OK
                loop {
                    if i >= GRENZE {
                        break;
                    }

                    i += 1;
                    // ...
                }

                // Fehler, for nutzt Iterator::next
                // for i in 0..4 {
                //     // ...
                // }
            }
        }
    }

    {
        // Eine Zufallszahl in einer konstanten Funktion
        const fn zufallszahl() -> i32 {
            use const_random::const_random;

            const ZUFALLSZAHL: i32 = const_random!(i32);
            ZUFALLSZAHL
        }

        // Vom Compiler berechnet
        const ZAHL_1: i32 = zufallszahl();
        println!("{ZAHL_1}");

        // Zur Laufzeit ausgewertet
        let zahl_2 = zufallszahl();
        println!("{zahl_2}");
    }

    {
        // Anonyme Funktionen
        // Eine anonyme Funktion mit Rückgabetyp
        let addiere = |a, b| -> i32 {
            a + b
        };
        let ergebnis = addiere(2, 4);

        // Ohne Rückgabe können die geschweiften Klammern entfallen
        let schreibe_ergebnis = |i| println!("{i}");
        schreibe_ergebnis(ergebnis);

        {
            // Schnellausführung
            println!(
                "2 + 3 = {}",
                (|a, b| a + b)(2, 3)
            ); // 2 + 3 = 5

            // Fehler nicht in konstanten Kontexten gestattet
            // const ERGEBNIS: i32 = (|a, b| a + b)(2, 3);
        }

        {
            // Wir können in stable nicht Rust Funktions-Traits
            // implementieren. Daher das Dummy-Trait Callable.
            trait Callable {
                fn call(&self) -> i32;
            }

            // Die anonyme Funktion wird zu einer Struktur
            struct Func_Addiere {
                a: i32,
                b: i32,
            }

            // Ein Funktions-Trait wird implementiert
            impl Callable for Func_Addiere {
                fn call(&self) -> i32 {
                    self.a + self.b
                }
            }

            let funktion = Func_Addiere { a: 2, b: 4 };
            // Der Aufruf der anonymen Funktion
            let ergebnis = funktion.call();
        }

        {
            // Jede anonyme Funktion hat einen eigenen Datentyp
            // let mut strings = Vec::new();
            // strings.push(|| format!("Hallo"));
            // strings.push(|| format!(", Rust"));

            {
                // Lösung:
                let mut strings: Vec<fn() -> String> = Vec::new();
                strings.push(|| format!("Hallo"));
                // OK
                strings.push(|| format!(", Rust"));

                fn noch_ein_string() -> String {
                    format!("!")
                }

                strings.push(noch_ein_string);

                for callback in strings {
                    println!("{}", callback())
                }
            }
        }

        {
            // Capture der Umgebung

            {
                // Copy und Move
                let addiere = |a, b| a + b;

                let arg_a = 3;
                let arg_b = 5;

                let ergebnis = addiere(arg_a, arg_b);

                // OK, i32 is Copy
                println!("Das Ergebnis aus {arg_a} und {arg_b} ist: {ergebnis}");

                let konkateniere = |s1: String, s2: String| -> String {
                    format!("{s1}{s2}")
                };

                let arg_s1 = "Hallo, ".to_string();
                let arg_s2 = "Rust".to_string();

                let str = konkateniere(arg_s1, arg_s2);

                // Fehler, die Werte hinter arg_s1 und arg_s2
                // wurden schon bewegt
                // println!("{arg_s1} und {arg_s2} zusammengefügt: {str}");
            }

            {
                // Variablen aus der Umgebung

                let arg_s1 = "Hallo, ".to_string();
                let arg_s2 = "Rust".to_string();

                let konkateniere = || format!("{arg_s1}{arg_s2}");

                let str = konkateniere();
                println!("{arg_s1} und {arg_s2} zusammengefügt: {str}");

                {
                    // Wenn die Besitzer nicht lang genug leben

                    let arg_s1 = "Hallo, ";
                    let arg_s2 = "Rust";

                    // Der Compiler wird sich hierüber beschweren
                    // let konkateniere = || println!("{arg_s1}{arg_s2}");
                    //
                    // // An einen Thread übergeben
                    // // Fehler, die Referenzen leben nicht lang genug
                    // std::thread::spawn(konkateniere)
                    //     // Warten, dass der Thread durchgelaufen ist
                    //     .join()
                    //     // Rückgabetyp ist Result, für den Fall, dass
                    //     // im Thread ein Fehler aufgetreten ist
                    //     .unwrap();

                    {
                        // Option 1: Statische Besitzer
                        static arg_s1: &str = "Hallo, ";
                        static arg_s2: &str = "Rust";

                        let konkateniere = || println!("{arg_s1}{arg_s2}");

                        // An einen Thread übergeben
                        std::thread::spawn(konkateniere)
                            // Warten, dass der Thread durchgelaufen ist
                            .join()
                            // Rückgabetyp ist Result, für den Fall, dass
                            // im Thread ein Fehler aufgetreten ist
                            .unwrap();
                    }

                    {
                        // Option 2: move
                        let arg_s1 = "Hallo, ";
                        let arg_s2 = "Rust";

                        // Move anwenden, statt Referenzen zu leihen
                        let konkateniere = move || println!("{arg_s1}{arg_s2}");

                        // An einen Thread übergeben
                        std::thread::spawn(konkateniere)
                            // Warten, dass der Thread durchgelaufen ist
                            .join()
                            // Rückgabetyp ist Result, für den Fall, dass
                            // im Thread ein Fehler aufgetreten ist
                            .unwrap();
                    }
                }

                {
                    // Explizite Lebenszeiten in anonymen Funktionen
                    let ausgabe =
                        |str1: &'static str,
                         str2: &'static str| println!("{str1}{str2}");

                    // OK, statische Lebenszeit
                    ausgabe("Hallo", ", Rust");

                    // Durch String in den Heap verschieben
                    let s1 = "Nur im".to_string();
                    let s2 = "Block".to_string();

                    // Fehler, Lebenszeit ist auf den Block begrenzt
                    // ausgabe(&s1, &s2);
                }
            }

            {
                // Anonyme Funktionen mit Seiteneffekten

                {
                    // Um Seiteneffekte zuzulassen, muss die Variable ebenfalls mut sein
                    let mut zeichenkette = "Hallo, Rust".to_string();
                    // Fehler, das kompiliert so nicht
                    let entferne = || zeichenkette.remove(zeichenkette.len() - 1);

                    // Fehler, entferne muss let mut sein
                    // println!(
                    //     "Nach dem Entfernen übrig: {zeichenkette}. Entfernt: {}.",
                    //     entferne()
                    // );

                    {
                        // Lösung
                        let mut entferne = || zeichenkette.remove(zeichenkette.len() - 1);
                        println!(
                            "Nach dem Entfernen übrig: {zeichenkette}. Entfernt: {}.",
                            entferne()
                        );
                    }
                }


                {
                    // Wann veränderliche Referenzen aktiv werden und wie lange sie so bleiben
                    let mut zeichenkette = "Hallo, Rust".to_string();
                    {
                        let mut entferne = || zeichenkette.remove(zeichenkette.len() - 1);
                        println!(
                            "Nach dem Entfernen übrig: {zeichenkette}. Entfernt: {}.",
                            entferne()
                        );
                        // Fehler
                        // println!(
                        //     "Nach dem Entfernen übrig: {zeichenkette}. Entfernt: {}.",
                        //     // Kann nicht ausführen, weil zeichenkette
                        //     // noch exklusiv von anonymer Funktion ausgeliehen
                        //     entferne()
                        // );
                    }

                    {
                        let mut entferne = || {
                            let entfernt = zeichenkette.remove(zeichenkette.len() - 1);
                            println!(
                                "Nach dem Entfernen übrig: {zeichenkette}. Entfernt: {}.",
                                entfernt
                            );
                        };

                        // Jetzt beliebig viele Aufrufe möglich
                        entferne();
                        entferne();
                        // ...
                    }
                }
            }
        }

        {
            // Funktions-Traits

            {
                // Funktionszeiger und Funktionstraits

                // - Eine Closure ohne Captures kann ein fn sein, andernfalls nicht
                fn gib_komma() -> String { ", ".to_string() }

                let mut strings: Vec<fn() -> String> = Vec::new();
                // OK, nichts aus der Umgebung
                strings.push(|| format!("Hallo"));
                strings.push(gib_komma);
                // OK, nichts aus der Umgebung
                strings.push(|| format!("Rust"));

                let bang: String = "!".into();
                // Fehler, greift auf die Umgebung zu
                // strings.push(|| bang);
            }

            {
                // FnOnce
                let string = "Hallo, Rust!".to_string();

                let in_bytes = || string.into_bytes();

                // OK, aber verbraucht das Objekt "string"
                // let bytes = in_bytes();
                // Fehler! in_bytes kann nur einmal ausgeführt werden
                // let nochmal_bytes = in_bytes();

                // Fehler, "string" ist wegen des Moves nicht mehr gültig
                // println!("{string}");

                {
                    // FnOnce hinter einer geteilten Referenz

                    // fn rufe_auf<F: FnOnce()>(funktion: &F) {
                    //     // Fehler! Der Aufruf würde "funktion" verbrauchen
                    //     funktion();
                    // }
                }
            }

            {
                // Fn
                // Vektor über Trait-Objekte von Fn
                let mut callbacks: Vec<&dyn Fn()> = Vec::new();
                let string = "Hallo, Rust";

                let gefangen = || println!("{string}");
                callbacks.push(&gefangen);

                let callback = || println!("Hallo");
                callbacks.push(&callback);
                // Eine anonyme Funktion landet standardmäßig auf dem Stack
                callbacks.push(&(|| println!(", ")));
                // Die Klammern oben nur zur Demonstration
                callbacks.push(&|| println!("Rust"));

                fn sag_hallo() {
                    println!("Hallo!");
                }
                // Eine reguläre Funktion implementiert auch Fn
                callbacks.push(&sag_hallo);

                // Anonyme Funktion vom Stack in den Heap schieben
                let im_heap = Box::new(
                    || println!("Grüße aus dem Heap")
                );

                // Dereferenziert auch automatisch (&im_heap)
                // * -> Dereferenziert zu anonymer Funktion
                // & -> Adresse der anonymen Funktion zurückgeben
                callbacks.push(&*im_heap);

                for callback in &callbacks {
                    callback();
                }
                // Die mehrmalige Ausführung von Fn-Funktionen
                for callback in &callbacks {
                    callback();
                }
            }

            {
                // FnMut
                let mut counter = 0;

                let mut tap = || counter += 1;

                fn doppel_tap<F>(mut callback: F) where F: FnMut() {
                    callback();
                    callback();
                }

                doppel_tap(tap);
                println!("Zählstand: {counter}"); // 2

                // Auch reguläre Funktionen können FnMut sein
                fn zweimal_ausfuehren<F>(
                    mut callback: F,
                    i: &mut i32,
                ) where F: FnMut(&mut i32) {
                    callback(i);
                    callback(i);
                }

                fn zaehle_hoch(i: &mut i32) {
                    *i += 1;
                }

                zweimal_ausfuehren(
                    zaehle_hoch,
                    &mut counter,
                );
                println!("Zählstand: {counter}"); // 4
            }

            {
                // Copy, Clone und Move

                {
                    // Copy
                    let a = 2;
                    let b = 2;

                    let kopie = || a + b;
                    let kopie_2 = kopie; // Copy

                    let doppelt = kopie() + kopie_2(); // 8
                    println!("{doppelt}");
                }

                {
                    // Clone
                    let str1 = "Hallo".to_string();
                    let str2 = "Rust".to_string();

                    let zusammen = || format!("{str1}, {str2}");
                    let zusammen_2 = zusammen; // Clone
                    println!("{}, {}", zusammen(), zusammen_2());
                    // Hallo, Rust, Hallo, Rust
                }

                {
                    // Move, wegen der veränderlichen Referenz
                    let mut str1 = "Hallo".to_string();
                    let str2 = "Rust".to_string();

                    // Eine exklusive Referenz kann nicht kopiert oder geklont werden!
                    let zusammen = || str1 + &str2;
                    let zusammen_2 = zusammen; // Move
                    // Fehler, Wert wurde aus "zusammen" bewegt!
                    // println!("{}, {}", zusammen(), zusammen_2());
                    // Hallo, Rust, Hallo, Rust
                }

                {
                    // Move
                    let a = 2;
                    let b = 2;

                    let kopie = move || a + b;
                    let kopie_2 = kopie; // Copy

                    let doppelt = kopie() + kopie_2(); // 8
                    println!("{doppelt}");
                    // OK, Originale sind nur kopiert worden
                    println!("{a} {b}");

                    // ...

                    let str1 = "Hallo".to_string();
                    let str2 = "Rust".to_string();
                    let zusammen = move || format!("{str1}, {str2}");
                    // OK dank Clone
                    let zusammen_2 = zusammen.clone(); // Clone verfügbar
                    // println!("{}, {}", zusammen(), zusammen_2());

                    // Die Originale erneut ausleihen
                    // println!("{str1}, {str2}"); // Fehler
                }
            }

            {
                // Test

                let foo = [
                    "Hallo".to_string(),
                    ",".to_string(),
                    "Rust".to_string()
                ];
                let zahlen_iterator: Vec<_> = foo
                    .into_iter()
                    .filter(|s| s.starts_with(','))
                    .collect();
                // println!("{foo:?}");

                fn foo<F>(mut f: F) where F: Fn() {
                    f();
                    f();
                }

                let str = String::new();
                // let bar = || { let _ = str.into_bytes();};

                // foo(bar);
            }


            // Temp
            let mut funktionen: Vec<fn(i32, i32) -> i32> = Vec::new();
            funktionen.push(|a, b| { a + b });

            let funktions_zeiger: fn(i32, i32) -> i32 = addiere;
        }
    }
}
