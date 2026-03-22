use std::borrow::Borrow;

#[derive(PartialEq, Debug)]
struct Kg(f64);

#[derive(PartialEq, Debug)]
struct Gewicht {
    wert: Kg,
}

impl Borrow<Kg> for Gewicht {
    fn borrow(&self) -> &Kg {
        &self.wert
    }
}

impl ToOwned for Kg {
    type Owned = Gewicht;

    fn to_owned(&self) -> Self::Owned {
        Gewicht {
            wert: Self { 0: self.0 },
        }
    }
}

pub fn main() {
    let gewicht = Gewicht { wert: Kg(83.2) };

    let g_ref: &Kg = gewicht.borrow();

    let gewicht_kopie: Gewicht = g_ref.to_owned();
    assert_eq!(gewicht, gewicht_kopie);
}
