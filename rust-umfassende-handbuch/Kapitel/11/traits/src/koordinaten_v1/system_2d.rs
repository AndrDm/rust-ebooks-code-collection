use std::collections::HashSet;
use std::ops::{RangeBounds, RangeInclusive, RangeTo};

trait Form {
    fn koordinaten(&self) -> Vec<Punkt>;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Punkt {
    x: i32,
    y: i32,
}

struct Linie {
    a: Punkt,
    b: Punkt,
}

struct Rechteck {
    a: Linie,
    b: Linie,
}

struct Kreis {
    m: Punkt,
    r: i32,
}

impl Form for Punkt {
    fn koordinaten(&self) -> Vec<Punkt> {
        // Punkt ist Copy, daher den Wert hinter
        // der Referenz ansprechen
        vec![*self]
    }
}


impl Linie {
    // Inhärenter impl-Block, da die Hilfsfunktion nicht
    // Teil der Trait-Implementierung von Form for Linie ist
    fn berechne_steigung(&self) -> i32 {
        let x_diff = self.a.x - self.b.x;
        let m = if x_diff == 0 {
            0
        } else {
            (self.a.y - self.b.y) / x_diff
        };

        m
    }
}

impl Form for Linie {
    fn koordinaten(&self) -> Vec<Punkt> {
        let a = &self.a;
        let b = &self.b;

        let mut koordinaten: Vec<Punkt> = Vec::new();

        if (a.x..=b.x).count() == 1 {
            // Keine Bewegung auf der x-Achse
            // Folge der y-Achse
            koordinaten.extend(
                (b.y..=a.y).map(|y| Punkt { x: a.x, y })
            );
            return koordinaten;
        }

        let steigung = self.berechne_steigung();
        for x in (a.x..=b.x).into_iter() {
            let y = steigung * x + a.y;
            koordinaten.push(Punkt { x, y });
        }

        koordinaten
    }
}

impl Form for Rechteck {
    fn koordinaten(&self) -> Vec<Punkt> {
        let mut punkte = Vec::new();
        let seite_a = self.a.koordinaten();
        punkte.extend(&seite_a);

        let seite_b = self.b.koordinaten();
        punkte.extend(&seite_b);

        let seite_c = seite_a
            .iter()
            .map(|p| {
                Punkt {
                    x: p.x,
                    y: p.y - (seite_b.len() - 1)  as i32,
                }
            });
        punkte.extend(seite_c);

        // Finde größte X-Koordinate auf Seite a
        let a_max_koordinate = seite_a
            .iter()
            .map(|p| p.x)
            .max()
            .unwrap_or(0);
        let seite_d = seite_b
            .iter()
            .map(|p| Punkt {
                x: a_max_koordinate,
                y: p.y,
            });
        punkte.extend(seite_d);

        punkte
    }
}

impl Form for Kreis {
    fn koordinaten(&self) -> Vec<Punkt> {
        let mut punkte = Vec::new();

        punkte.push(Punkt { x: self.m.x, y: self.m.y + self.r });
        punkte.push(Punkt { x: self.m.x, y: self.m.y - self.r });

        punkte.push(Punkt { x: self.m.x + self.r, y: self.m.y });
        punkte.push(Punkt { x: self.m.x - self.r, y: self.m.y });

        punkte
    }
}

struct Koordinatenfeld {
    punkte: HashSet<Punkt>,
    x_achse: RangeInclusive<i32>,
    y_achse: RangeInclusive<i32>,
}

impl Koordinatenfeld {
    fn new_10_10(formen: Vec<&dyn Form>) -> Koordinatenfeld {
        let mut punkte = HashSet::<Punkt>::new();

        for form in formen {
            // Achtung, extend ist nur verfügbar, wenn T
            // in HashSet<T> die Traits Eq und Hash implementiert!
            // Ansonsten wird der Compiler melden, dass extend
            // nicht bekannt ist.
            punkte.extend(form.koordinaten().iter());
        }

        Koordinatenfeld {
            punkte,
            x_achse: 0..=10,
            y_achse: 0..=10,
        }
    }

    fn zeichne(&self) {
        for y in self.y_achse.clone().rev() {
            for x in self.x_achse.clone() {
                let hat_punkt = self.punkte
                    .iter()
                    .find(|p| **p == Punkt { x, y });

                if let Some(_) = hat_punkt {
                    print!("+\t");
                } else {
                    if x == 0 {
                        print!("|\t");
                        continue;
                    } else if y == 0 {
                        // Ein langer Bindestrich
                        print!("\u{2015}\t");
                    } else {
                        print!("\t");
                    }
                }
            }

            println!();
        }
    }
}

pub fn main() {
    {
        // Leeres Koordinatenfeld
        let koordinatenfeld = Koordinatenfeld {
            punkte: HashSet::new(),
            x_achse: 0..=10,
            y_achse: 0..=10,
        };

        koordinatenfeld.zeichne();
        println!();
    }

    let linie_1 = Box::new(Linie {
        a: Punkt { x: 6, y: 10 },
        b: Punkt { x: 9, y: 10 },
    });

    let feld = Koordinatenfeld::new_10_10(vec![
        &*linie_1,
        &Rechteck {
            a: Linie {
                a: Punkt { x: 4, y: 4 }, b: Punkt { x: 8, y: 4 }
            },
            b: Linie {
                a: Punkt { x: 4, y: 4 }, b: Punkt { x: 4, y: 1 }
            },
        },
        &Rechteck {
            a: Linie {
                a: Punkt { x: 2, y: 10 }, b: Punkt { x: 3, y: 10 }
            },
            b: Linie {
                a: Punkt { x: 2, y: 10 }, b: Punkt { x: 2, y: 1 }
            },
        },
        &Kreis {
            m: Punkt { x: 7, y : 7 },
            r: 1,
        },
    ]);

    feld.zeichne();
}