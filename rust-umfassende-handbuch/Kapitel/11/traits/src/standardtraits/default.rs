use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
struct Punkt {
    x: i32,
    y: i32,
}

impl Default for Punkt {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn erstelle<T: Default>() -> T {
    T::default()
}

#[derive(PartialEq, Debug)]
struct Person {
    name: Rc<RefCell<Option<String>>>,
    // ...
}

pub fn main() {
    let a: Punkt = erstelle();
    let str: String = erstelle();
    let i: i32 = erstelle();

    let a = Punkt::default(); // 0,0
    let b: Punkt = Default::default(); // 0,0

    assert_eq!(a, b); // true
    assert_eq!(a, Punkt { x: 0, y: 0 }); // true

    // Vereinfacht durch Default
    let p_einfach = Person {
        name: Default::default(),
    };

    let p_kombiniert = Person {
        name: Rc::new(RefCell::new(Option::None)),
    };

    assert_eq!(p_einfach, p_kombiniert);
}
