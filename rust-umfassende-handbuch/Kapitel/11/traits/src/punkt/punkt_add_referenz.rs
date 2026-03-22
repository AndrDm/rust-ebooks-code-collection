use super::Punkt;

// Vollständig mit 'a
// impl<'a> std::ops::Add<&'a Punkt> for &'a Punkt {
//     type Output = Punkt;
//
//     fn add(self, rhs: &'a Punkt) -> Self::Output {
//         Punkt {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//         }
//     }
// }

// Self mit 'a
impl<'a> std::ops::Add for &'a Punkt {
    type Output = Punkt;

    fn add(self, rhs: Self) -> Self::Output {
        Punkt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> std::ops::Add<i32> for &'a Punkt {
    type Output = Punkt;

    fn add(self, rhs: i32) -> Self::Output {
        Punkt {
            x: self.x + rhs,
            y: self.y + rhs,
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

    static PS: Punkt = Punkt { x: 1, y: 2 };

    let p3 = &PS + &Punkt { x: 2, y: 1 };
}
