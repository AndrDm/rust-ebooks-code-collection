use std::fmt::Formatter;

mod assoziierter_typ;
mod funktionen_vs_methoden;
mod impl_reverse_generic;
mod konstanten;
mod koordinaten_v1;
mod methoden;
mod punkt;
mod standardtraits;
mod supertraits;
mod trait_objekte;
mod sichtbarkeiten;


fn main() {
    trait Form {
        fn koordinaten(&self) -> Vec<Punkt>;
    }

    // Der lokale Datentyp "Punkt"
    #[derive(Clone, Debug, PartialEq, Eq, Hash)]
    struct Punkt {
        x: i32,
        y: i32,
    }

    // Später in eine Enumeration verwandeln?
    struct Kreis {
        p: Punkt,
        r: f64,
    }
    struct Linie {
        a: Punkt,
        b: Punkt,
    }
    struct Rechteck {
        a: Linie,
        b: Linie,
    }

    {
        // Nur Trait-Elemente im Trait-impl-Block
        // impl Form for Linie {
        //     // Fehler, nicht mit Form assoziiert
        //     fn hallo_linie() {
        //         println!("Hallo, Linie");
        //     }

        // // Fehler
        // fn punkt_a(&self) -> Punkt {
        //     self.a
        // }

        // // Fehler
        // const MAX: i32 = 5;

        // }
    }

    {
        // Orphan Rule
        {
            // Ok, fremdes Trait aber crate-lokaler Datentyp
            impl Copy for Punkt {}

            // Fremdes Trait und fremder Datentyp
            // Fehler, sowohl Trait als auch Datentyp sind fremd
            // impl Copy for String {}
        }

        {
            // Lokales Trait, fremder Typ
            trait MeinLokalesTrait {
                // ...
            }

            impl MeinLokalesTrait for String {
                // ...
            }
        }

        {
            // Trait und Typparameter des Traits
            use std::ops::Add;
            // Fehler
            // impl Add<bool> for i32 {
            //     // ...
            // }

            // Ok, Punkt ist ein lokaler Typ als Typparameter
            impl Add<Punkt> for i32 {
                type Output = Punkt;

                fn add(self, rhs: Punkt) -> Self::Output {
                    Punkt {
                        x: self + rhs.x,
                        y: self + rhs.y,
                    }
                }
            }

            let p = 5 + Punkt { x: 1, y: 2 };
            // Punkt { x: 6, y: 7 }
            println!("Skalar + Punkt: {p:?}");
        }
    }

    // Form mit assoziiertem Datentyp
    assoziierter_typ::main();
    punkt::main();

    konstanten::main();
    methoden::main();

    funktionen_vs_methoden::main();

    sichtbarkeiten::main();

    trait_objekte::main();
    impl_reverse_generic::main();
    koordinaten_v1::main();
    standardtraits::main();
}
