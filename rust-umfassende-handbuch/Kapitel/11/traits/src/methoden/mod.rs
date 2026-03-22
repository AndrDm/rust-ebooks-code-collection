trait Form {
    // Wir fügen zur Trait-Grenze
    // das Trait Default hinzu.
    type Koordinate: std::ops::Add + Default;

    fn start(&self) -> Self::Koordinate {
        Self::Koordinate::default()
    }
}

#[derive(
    // Ermöglicht die Debug-Ausgabe mit ":?"
    Debug,
    // Default lässt sich automatisch
    // implementieren.
    Default,
)]
struct Punkt {
    x: i32,
    y: i32,
}

impl Form for Punkt {
    type Koordinate = Punkt;

    // Benutzt die Standardimplementierung
    // von start in Trait Form.
    // Unteren Code einkommentieren, um zu
    // überschreiben.
    // fn start(&self) -> Self::Koordinate {
    //     // Start soll bei (2, 2) liegen
    //     Punkt { x: 2, y: 2 }
    // }
}

impl std::ops::Add for Punkt {
    type Output = Punkt;
    // Oder:
    // type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Punkt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub(super) fn main() {
    let p = Punkt { x: 2, y: 3 };
    println!("Start: {:?}", p.start());
    // Start: Punkt { x: 0, y: 0 }
}