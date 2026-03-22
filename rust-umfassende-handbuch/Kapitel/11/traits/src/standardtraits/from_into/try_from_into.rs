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

impl TryFrom<f64> for Kilo {
    type Error = String;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value >= 0.0 {
            Ok(Self { 0: value })
        } else {
            Err("Muss größer 0 sein".into())
        }
    }
}

fn lese_gewichtswert<T: TryInto<Kilo>>(gewicht: T) {
    let gewicht = gewicht.try_into();
    if let Ok(gewicht) = gewicht {
        let wert = gewicht.0;
        // ...
    }
}

pub fn main() {
    lese_gewichtswert(Pfund(177.9));
    lese_gewichtswert(Kilo(87.1));

    let gewicht_kg = Kilo(84.2);
    use std::convert::Infallible;
    let gewicht_pfund: Result<Pfund, Infallible> = TryFrom::try_from(gewicht_kg);
    println!("Größe TryFrom: {}", std::mem::size_of_val(&gewicht_pfund));
    let gewicht_pfund = gewicht_pfund.unwrap();
    println!("Größe TryFrom: {}", std::mem::size_of_val(&gewicht_pfund));
}
