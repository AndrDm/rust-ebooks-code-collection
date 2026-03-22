#[derive(Debug)]
struct Punkt {
    x: i32,
    y: i32,
}

impl<'a, 'b> std::ops::Add<&'b Punkt> for &'a Punkt {
    type Output = Punkt;

    fn add(self, rhs: &'b Punkt) -> Self::Output {
        Punkt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub(super) fn main() {
    let p1 = Punkt { x: 2, y: 4 };
    let p2 = Punkt { x: 3, y: 5 };
    let p3 = &p1 + &p2;
    // ...
    println!("{p3:?}");
    // Ok
    let p4 = &p1 + &Punkt { x: 0, y: 0 };
    // Ok
    let p5 = &Punkt { x: 0, y: 0 } + &p2;
    println!("{p4:?}, {p5:?}");
}