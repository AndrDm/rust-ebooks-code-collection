
trait Form {
    type Koordinate: std::ops::Add;

    // In der Implementierung setzen
    const MAX_X: Self::Koordinate;
    // In der Implementierung setzen
    const MAX_Y: Self::Koordinate;

    // Eine Konstante direkt definieren
    const KREISZAHL: f64 = 3.14159;

    // ...
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

    const KREISZAHL: f64 = 9.0;

    // ...
}

impl Punkt {
    const BESONDERS: i32 = 2;
}

pub(super) fn main() {

    // Zugriff über den implementierenden Datentyp
    println!("{}",   Punkt::KREISZAHL);
    println!("{:?}", Punkt::MAX_X);
    println!("{:?}", Punkt::MAX_Y);

    // Zugriff über das Trait
    // Fehler, so geht es leider nicht!
    // println!("{}",   Form::KREISZAHL);
    // println!("{:?}", Form::MAX_X);
    // println!("{:?}", Form::MAX_Y);

    // Zugriff über das Trait
    // Richtig: Die Konstante über den
    // implementierenden Datentyp auflösen.
    println!("{}",   <Punkt as Form>::KREISZAHL);
    println!("{:?}", <Punkt as Form>::MAX_X);
    println!("{:?}", <Punkt as Form>::MAX_Y);


}