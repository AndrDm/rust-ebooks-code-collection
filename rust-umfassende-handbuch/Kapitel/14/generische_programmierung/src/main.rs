#![allow(unused_variables)]

fn main() {
    {
        // Überblick Typparameter
        fn bilde_summe<T>(v: Vec<T>) /*-> T */
        {
            // ...
        }

        {
            // Mit Formatierer

            fn bilde_summe<T: std::fmt::Display>(v: Vec<T>) -> T {
                for element in v {
                    println!("{element}");
                }
                // ...
                todo!()
            }
        }

        let v = vec![1, 2, 3];

        // T: i32, ergebnis: i32
        let ergebnis = bilde_summe(v);

        let ergebnis = bilde_summe::<f64>(Vec::new());
    }

    {
        // Generische Konstanten
        struct Puffer<const N: usize, T> {
            speicher: [T; N],
        }

        let p = Puffer::<5, i32> {
            speicher: [1, 2, 3, 4, 5],
        };
        // Fehler, muss Länge 5 aufweisen
        // let p = Puffer::<5, i32> { speicher: [1, 2, 3, 4]};

        let p = Puffer {
            speicher: ["Hallo", "Rust"],
        };
        assert_eq!(p.speicher[0], "Hallo");
        assert_eq!(p.speicher[1], "Rust");
    }

    {
        // Reihenfolge in generischen Parametern
        struct Puffer<'a, const N: usize, T> {
            _p: std::marker::PhantomData<&'a bool>,
            speicher: [T; N],
        }
    }

    {
        // Generische Elemente

        // Die Konstante tritt hier als "M" auf.
        // Sie hat nicht mit "N" in Puffer zu tun.
        // impl<const M: usize, T, R> Puffer<M, T> {
        //     // Fehler: Den Typparameter R nutzt nur die Funktion,
        //     // nicht aber die Struktur Puffer
        //     fn callback(r: R) -> R {
        //         // ...
        //     }
        // }

        {
            // Assoziierter Datentyp

            struct PufferElement;

            impl std::ops::Add<i32> for PufferElement {
                // ...
                type Output = i32;

                fn add(self, rhs: i32) -> Self::Output {
                    todo!()
                }
            }

            impl std::ops::Add<f64> for PufferElement {
                // ...
                type Output = f64;

                fn add(self, rhs: f64) -> Self::Output {
                    todo!()
                }
            }

            impl std::ops::Deref for PufferElement {
                // ...
                type Target = ();

                fn deref(&self) -> &Self::Target {
                    todo!()
                }
            }
        }

        {
            // Assoziierten Datentyp generisch setzen
            struct Puffer<const N: usize, T> {
                speicher: [T; N],
            }

            impl<const M: usize, T: Iterator<Item = U>, U: std::ops::Add> Puffer<M, T> {
                fn zufallselement(&self) -> U {
                    // ...
                    todo!()
                }
            }
        }
    }

    {
        // Trait-Grenzen

        {
            // Unvollständige Version
            // fn bilde_summe<T: std::fmt::Display>(v: Vec<T>) -> T {
            //     // Fehler, T ist nicht Default
            //     let mut summe = T::default();
            //
            //     for element in v {
            //         println!("{element}");
            //         // Fehler: T ist nicht Add
            //         summe += element;
            //     }
            //
            //     summe
            // }
        }

        {
            // Vollständige Version
            fn bilde_summe<T: std::fmt::Display + std::ops::AddAssign + Default>(v: Vec<T>) -> T {
                // Fehler, T ist nicht Default
                let mut summe = T::default();

                for element in v {
                    println!("{element}");
                    // Fehler: T ist nicht AddAssign
                    summe += element;
                }

                summe
            }

            let summe = bilde_summe::<i32>(vec![1, 2, 3]);
            assert_eq!(summe, 6);
        }

        {
            // impl Trait
            fn bilde_summe<T: std::fmt::Display + std::ops::AddAssign + Default>(v: Vec<T>) -> T {
                // Fehler, T ist nicht Default
                let mut summe = T::default();

                for element in v {
                    println!("{element}");
                    // Fehler: T ist nicht AddAssign
                    summe += element;
                }

                summe
            }
        }

        {
            // where-Teil
            fn bilde_summe<T>(v: Vec<T>) -> T
            where
                T: std::fmt::Display + std::ops::AddAssign + Default,
            {
                // Fehler, T ist nicht Default
                let mut summe = T::default();

                for element in v {
                    println!("{element}");
                    // Fehler: T ist nicht AddAssign
                    summe += element;
                }

                summe
            }

            trait FormularZustand<T>
            where
                T: PartialEq,
            {
                type Anmerkung;
            }

            enum Personendaten<S, Z, A>
            where
                S: Iterator,
                // FormularZustand<T>: Generischen Parameter "T"
                // und assoziierten Datentyp
                // "FormularZustand::Anmerkung" bestimmen.
                Z: FormularZustand<String, Anmerkung = A>,
                // Trait-Grenze für assoziierten Datentyp
                // "FormularZustand::Anmerkung"
                A: Into<String>,
            {
                PositionenAusgefuellt(S, Z),
                Leer,
            }
        }
    }

    {
        // Blanket Implementierungen
        trait LogDebug {
            fn debug_log(&self) {
                // ... schreibe in Debug-Ausgabe
                println!("LogDebug!");
            }
        }

        {
            // impl<T> LogDebug for T
            //     where
            //         T: std::fmt::Debug
            // {
            //     // ...
            // }
            //
            // let s = String::new();
            // s.debug_log();
            //
            // // Fehler
            // let s_ref = &s;
            // s_ref.debug_log();
        }

        {
            // Nur, wenn "T" in einem Referenzzähler ist (siehe Abschnitt 18.2.3)
            impl<T> LogDebug for std::rc::Rc<T>
                where
                    T: std::fmt::Debug,
            {
                // ...
            }
        }

        {
            // T muss in einer Box sein und außerdem
            // einen Iterator über i32 aufweisen
            impl<T> LogDebug for Box<T>
                where
                    T: std::fmt::Debug + Iterator<Item=i32>,
            {
                // ...
            }
        }


    }
}
