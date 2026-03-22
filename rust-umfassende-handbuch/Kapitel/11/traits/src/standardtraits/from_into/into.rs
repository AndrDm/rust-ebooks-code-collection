struct Kilo(f64);
struct Pfund(f64);

impl From<Kilo> for Pfund {
    fn from(value: Kilo) -> Self {
        Self(value.0)
    }
}

impl From<Pfund> for Kilo {
    fn from(value: Pfund) -> Self {
        Self(value.0)
    }
}

fn lese_gewichtswert<T: Into<Kilo>>(gewicht: T) {
    let gewicht: Kilo = gewicht.into();
    let wert = gewicht.0;
    // ...
}

pub fn main() {
    let gewicht_kg = Kilo(84.2);

    let gewicht_lb: Pfund = gewicht_kg.into();

    let wieder_kg: Kilo = gewicht_lb.into();

    lese_gewichtswert(Pfund(177.9));
    lese_gewichtswert(Kilo(87.1));

    let kilo: Kilo = Kilo(84.3).into();
    let pfund: Pfund = Pfund(186.5).into();

    let kilo_from: Kilo = From::from(kilo);
    let pfund_from: Pfund = From::from(pfund);
}
