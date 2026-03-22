use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Clone)]
enum Gewicht {
    Kilo(f64, String),
    Pfund(f64, String),
    Fehlmessung {
        nutzer: String,
        datum: String,
        meldung: String,
    },
}

impl Gewicht {
    // Assoziierte Konstanten
    const EIN_KG_IN_PFUND: f64 = 2.2046226218;
    const EIN_PFUND_IN_KG: f64 = 0.45359237;

    pub fn kg_in_pfund() -> f64 {
        Self::EIN_KG_IN_PFUND
    }
    pub fn pfund_in_kg() -> f64 {
        Self::EIN_PFUND_IN_KG
    }

    // Methoden
    fn ist_fehler(&self) -> bool {
        if let Gewicht::Fehlmessung { .. } = self {
            true
        } else {
            false
        }
    }

    fn ist_kg(&self) -> bool {
        if let Gewicht::Kilo(_, _) = self {
            true
        } else {
            false
        }
    }

    fn ist_pfund(&self) -> bool {
        if let Gewicht::Pfund(_, _) = self {
            true
        } else {
            false
        }
    }
}

// Implementierung von Traits
impl Add for Gewicht {
    type Output = Gewicht;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.ist_fehler() {
            return self;
        }

        match self {
            Gewicht::Kilo(kg, d) => {
                let rhs_kg = if rhs.ist_kg() {
                    let Gewicht::Kilo(wert, _) = rhs else { panic!("Typ vorher geprüft")};
                    wert
                } else {
                    let Gewicht::Pfund(wert, _) = rhs else { panic!("Typ vorher geprüft")};

                    wert * Gewicht::EIN_PFUND_IN_KG
                };

                Gewicht::Kilo(kg + rhs_kg, d)
            }
            Gewicht::Pfund(lb, d) => {
                // ... analog zu Kilo
                let rhs_kg = if rhs.ist_pfund() {
                    let Gewicht::Pfund(wert, _) = rhs  else { panic!("Typ vorher geprüft")};
                    wert
                } else {
                    let Gewicht::Kilo(wert, _) = rhs  else { panic!("Typ vorher geprüft")};

                    wert * Gewicht::EIN_KG_IN_PFUND
                };

                Gewicht::Pfund(lb + rhs_kg, d)
            }
            Gewicht::Fehlmessung { .. } => self,
        }
    }
}

impl Display for Gewicht {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gewicht::Kilo(g, d) => f.write_fmt(format_args!("{g}kg gelesen am {d}")),
            Gewicht::Pfund(g, d) => f.write_fmt(format_args!("{g}lb gelesen am {d}")),
            Gewicht::Fehlmessung {
                nutzer,
                datum,
                meldung,
            } => f.write_fmt(format_args!("Fehler bei Messung: {meldung}")),
        }
    }
}

pub fn main() {
    // Wir ignorieren das Datum in diesen Testläufen
    let ist_pfund = Gewicht::Kilo(64.0, "".into()).ist_pfund(); // false
    let kg = Gewicht::Kilo(64.0, "".into());
    let ist_kg = kg.ist_kg(); // true

    let pfund = Gewicht::Pfund(2.0, "".into());
    let ist_pfund = pfund.ist_pfund(); // true
    let ist_kg = Gewicht::Pfund(2.0, "".into()).ist_kg(); // false

    let fehler = Gewicht::Fehlmessung {
        nutzer: "".into(),
        datum: "".into(),
        meldung: "Fehler aufgetreten".into(),
    };
    let ist_fehler = fehler.ist_fehler(); // true

    let kg = Gewicht::Kilo(64.0, "".into());
    let pfund = Gewicht::Pfund(2.0, "".into());

    println!("Gesamtgewicht in Kilo: {}", kg + pfund);

    let kg = Gewicht::Kilo(64.0, "".into());
    let pfund = Gewicht::Pfund(2.0, "".into());
    println!("Gesamtgewicht in Pfund: {}", pfund + kg);

    let kg = Gewicht::Kilo(64.0, "".into());
    let fehler = Gewicht::Fehlmessung {
        nutzer: "".into(),
        datum: "".into(),
        meldung: "Fehler aufgetreten".into(),
    };
    println!("Gesamtgewicht Fehler: {}", fehler + kg);
}
