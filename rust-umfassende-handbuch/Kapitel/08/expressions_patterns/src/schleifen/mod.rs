pub fn main() {
    {
        // Label-Block
        // let bis_ende = 'ein_block: {
        //     println!("Hallo");
        //
        //     println!("Fortfahren mit [J], oder beenden mit [N]");
        //     let mut ergebnis = String::new();
        //     let _ = std::io::stdin().read_line(&mut ergebnis);
        //
        //     if ergebnis.trim() != "J" {
        //         println!("Sie beenden das Programm vorzeitig");
        //         break 'ein_block false;
        //     }
        //
        //     true
        // };
        //
        // if bis_ende {
        //     println!("Sie haben das Programm bis zum Ende ausgeführt!");
        // }
    }

    {
        // loop
        let mut a = 0;
        let mut b = 0;

        'a: loop {
            'b: loop {
                println!("{a}{b}");
                b += 1;
                if b > 9 {
                    if a > 8 {
                        break 'a;
                    }
                    b = 0;
                    break 'b;
                }
            }
            a += 1;
        }
    }

    {
        // loop und continue
        let mut zaehler = 0;
        loop {
            zaehler += 1;

            match zaehler {
                zahl @ (1 | 3 | 5 | 7) => {
                    println!("Primzahl: {zahl}");
                    continue;
                },
                _ => ()
            }

            if zaehler >= 10 {
                break;
            }
        }

        let mut zaehler = 0;

        'aussen: loop {
            zaehler += 1;
            if zaehler >= 10 {
                break
            }

            loop {
                println!("Durchgang: {zaehler}");
                continue 'aussen;
            }
        }


        {
            // Die loop kann als Ausdruck auch einen Wert zurückgeben

            let mut versuche = 0;

            let ergebnis = loop {
                versuche += 1;

                let x = rand::random::<i32>();
                if x % 2 == 0 {
                    break x;
                }
            };

            println!("Nach {versuche} Versuchen ist die gerade Zahl: {ergebnis}");
        }

    }

    {
        // while
        let mut i = 0;
        while (0..=10).contains(&i) {
            println!("while {i} von 10");
            i += 1;
        }

        {
            // while let
            let mut str = Vec::from("Hallo, Rust");
            str.reverse();
            let mut zaehler = 1;
            let laenge = str.len();

            // Ausführen, solange str.pop() nicht None ist
            while let Some(c) = str.pop() {

                println!(
                    "Buchstabe {} von {}: {}",
                    zaehler,
                    laenge,
                    char::from(c)
                );

                zaehler += 1;
            };

            {
                // Als loop-match Kombination
                let mut str = Vec::from("Hallo, Rust");
                str.reverse();
                let mut zaehler = 1;
                let laenge = str.len();

                // ...
                loop {
                    // Ausführen, solange str.pop() nicht None ist
                    match str.pop() {
                        Some(c) => {
                            println!(
                                "Buchstabe {} von {}: {}",
                                zaehler,
                                laenge,
                                char::from(c)
                            );
                            zaehler += 1;
                        }
                        _ => break,
                    }
                }
            }
        }
    }

    {
        // Die for-Schleife

        for i in 1..=3 {
            print!("{i}");
        } // 123
        println!();

        // Fehler, das Muster erfasst nicht
        // jeden möglichen Wert.
        // for i @ (1 | 3 | 5 | 7) in 0..=10 {
        //     println!("Primzahl: {i}");
        // }

        let worte = [
            "Hallo", "0", ",", "1", " ", "Rust", "2", "!", "Hallo"
        ];

        let mut str = String::new();

        'aussen: for i in 1.. {

            str.clear();

            'innen: for wort in worte {
                if let Ok(i) = wort.parse::<i32>() {
                    println!("Übersprungen: {i}"); // 0, 1, 2
                    continue;
                }

                if wort == "!" && i < 3 {
                    break 'innen;
                } else if wort == "!" {
                    break 'aussen;
                }

                str += wort;
            }

            println!("Ausgabe {i}: {str}");
        };

        println!("Letzte Ausgabe: {str}");
    }
}