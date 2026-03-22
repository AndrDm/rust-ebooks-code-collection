// Das Crate zum Projekt hinzufügen
// cargo add enum-as-inner

use enum_as_inner::EnumAsInner;

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

#[derive(EnumAsInner)]
enum Form {
    Punkt(Punkt),
    Linie(Linie),
}

pub fn main() {
    let p = Form::Punkt(Punkt { x: 1, y: 2 });
    let l = Form::Linie(Linie {
        a: Punkt { x: 1, y: 2 },
        b: Punkt { x: 3, y: 4 },
    });

    let punkt: &Punkt = p.as_punkt().unwrap();
    let linie: &Linie = l.as_linie().unwrap();
}
