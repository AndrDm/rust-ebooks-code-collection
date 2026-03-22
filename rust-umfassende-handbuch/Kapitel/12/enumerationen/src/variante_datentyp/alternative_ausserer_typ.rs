#[derive(Debug)]
struct Punkt {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Linie {
    a: Punkt,
    b: Punkt,
}

enum Form {
    Punkt(Punkt),
    Linie(Linie),
}

impl TryFrom<Form> for Punkt {
    type Error = ();

    fn try_from(value: Form) -> Result<Self, Self::Error> {
        match value {
            Form::Punkt(p) => Ok(p),
            Form::Linie(_) => Err(()),
        }
    }
}

impl TryFrom<Form> for Linie {
    type Error = ();

    fn try_from(value: Form) -> Result<Self, Self::Error> {
        match value {
            Form::Punkt(_) => Err(()),
            Form::Linie(l) => Ok(l),
        }
    }
}

pub fn main() {
    let p = Form::Punkt(Punkt { x: 1, y: 2 });
    let l = Form::Linie(Linie {
        a: Punkt { x: 1, y: 2 },
        b: Punkt { x: 3, y: 4 },
    });

    let punkt: Punkt = p.try_into().unwrap();
    println!("{punkt:?}");
    // Punkt { x: 1, y: 2 }

    let linie: Linie = l.try_into().unwrap();
    println!("{linie:?}");
    // Linie { a: Punkt { x: 1, y: 2 }, b: Punkt { x: 3, y: 4 } }
}
