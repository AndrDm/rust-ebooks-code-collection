mod punkt_add_referenz;
mod add_referenz_lebenszeiten;

trait Form {
    // Bevor eine Trait-Grenze eingefügt wird
    // type Koordinate;
    type Koordinate: std::ops::Add;

    // In der Implementierung setzen
    const MAX_X: Self::Koordinate;
    // In der Implementierung setzen
    const MAX_Y: Self::Koordinate;

    const KREISZAHL: f64 = 3.14159;

    fn addiere(
        a: Self::Koordinate,
        b: Self::Koordinate,
    ) -> Self::Koordinate;
}

#[derive(Debug, Copy, Clone)]
struct Punkt {
    x: i32,
    y: i32,
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

impl Form for Punkt {
    type Koordinate = Punkt;

    const MAX_X: Self::Koordinate = Punkt { x: 100, y: 0 };
    const MAX_Y: Self::Koordinate = Punkt { x: 0, y: 100 };

    fn addiere(
        a: Self::Koordinate,
        b: Self::Koordinate,
    ) -> Self::Koordinate {
        a + b
    }
}

pub fn main() {
    let p1 = Punkt { x: 2, y: 4 };
    let p2 = Punkt { x: 3, y: 5 };

    let p3 = p1 + p2;
    // ...
    println!("{p3:?}");

    // Fehler, p1 bereits bewegt
    // let p4 = p1 + Punkt { x: 0, y: 0 };
    // Fehler, p2 bereits bewegt
    // let p5 = Punkt { x: 0, y: 0 } + p2;

    // Stattdessen mit Referenzen arbeiten
    punkt_add_referenz::main();

    // Unterschiedliche Lebenszeiten
    add_referenz_lebenszeiten::main();
}

// Beispiel: Inhärente Implementierungen unterstützen
// assoziierte Datentypen nicht.
// impl Punkt {
//     // Fehler, nicht unterstützt
//     type P = i32;
// }