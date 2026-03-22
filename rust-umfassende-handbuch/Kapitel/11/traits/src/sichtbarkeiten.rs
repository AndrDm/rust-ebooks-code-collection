mod aussen {
    pub mod innen {
        pub trait Form {
            fn zeichne();
        }

        struct Kreis;
        // Ok, gleiches Modul
        impl Form for Kreis {
            fn zeichne() {
                // ...
                todo!()
            }
        }
    }

    struct Linie;

    // Fehler, "Form" ist privat
    impl innen::Form for Linie {
        fn zeichne() {
            // ...
            todo!()
        }
    }
}

struct Punkt;
// Fehler, "Form" ist privat
impl aussen::innen::Form for Punkt {
    fn zeichne() {
        // ...
        todo!()
    }
}

pub fn main() {
    // Fehler, "zeichne" ist unbekannt
    // Punkt::zeichne();

    {
        use aussen::innen::Form;

        Punkt::zeichne();
    }
}