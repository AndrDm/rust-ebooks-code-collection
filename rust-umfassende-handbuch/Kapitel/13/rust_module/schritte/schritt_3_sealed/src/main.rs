

mod formen {

    pub struct Punkt {
        x: i32,
        y: i32,
    }

    impl Punkt {
        pub fn new(x: i32, y: i32) -> Punkt {
            Punkt { x, y }
        }
    }

    struct Linie {
        pub a: Punkt,
        pub b: Punkt,
    }

    impl phantom_modul::SealedMarker for Punkt {}

    impl Form for Punkt {}

    impl phantom_modul::SealedMarker for Linie {}

    impl Form for Linie {}

    // Soll nach außen Sichtbar, aber nicht implementierbar sein
    pub trait Form: phantom_modul::SealedMarker {}

    mod phantom_modul {
        pub trait SealedMarker {}
    }
}

fn main() {
    // Ok, Trait ist sichtbar
    let form: &dyn formen::Form = &formen::Punkt::new(2, 4);
    // ...

    // Eine fremde Datenstruktur soll Form implementieren
    struct Kreis;

    // Eigentlich Ok, aber Super-Trait fehlt! Siehe unten
    // impl formen::Form for Kreis {}
    // Fehler, "phantom_modul" ist nicht nach außen sichtbar!
    // impl formen::phantom_modul::SealedMarker for Kreis {}
}