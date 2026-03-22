// trait Form: Copy + std::fmt::Debug {
//     // ...
// }

trait Form where Self: Copy + std::fmt::Debug {
    // ...
}

struct Punkt {
    x: i32,
    y: i32,
}

impl Form for Punkt {}

impl Clone for Punkt {
    fn clone(&self) -> Self {
        // ...
        todo!()
    }
}

impl Copy for Punkt {

}

impl std::fmt::Debug for Punkt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // ...
        todo!()
    }
}

impl Punkt {

}

pub(super) fn main() {
}