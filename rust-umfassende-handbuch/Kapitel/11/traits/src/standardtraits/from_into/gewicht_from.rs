struct Kilo(f64);
struct Pfund(f64);

enum Gewicht {
    Kilo(Kilo),
    Pfund(Pfund),
}

impl From<Kilo> for Gewicht {
    fn from(value: Kilo) -> Self {
        Self::Kilo(value)
    }
}

impl From<Pfund> for Gewicht {
    fn from(value: Pfund) -> Self {
        Self::Pfund(value)
    }
}

pub fn main() {
    let kg = Kilo(82.5);
    let gewicht = Gewicht::from(kg);

    let lb = Pfund(134.1);
    let gewicht: Gewicht = From::from(lb);
}
