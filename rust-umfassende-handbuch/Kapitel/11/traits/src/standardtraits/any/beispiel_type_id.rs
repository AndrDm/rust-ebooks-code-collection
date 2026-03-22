use std::any::{Any, TypeId};

struct Person<'a> {
    name: &'a str,
}

pub fn main() {
    // 'a erhält Lebenszeit 'static, weil
    // "Freda" den Datentyp &'static str hat
    let p = Person { name: "Freda" };
    // Der Aufruf legt Person die Forderung auf,
    // mindestens die Lebenszeit 'static aufzuweisen
    let type_id = p.type_id();
    // Ok
    assert_eq!(type_id, TypeId::of::<Person>());

    let name = String::from("Fred");
    // name ist eine Stackvariable mit Lebenszeit 'a,
    // wobei 'a kürzer als 'static ist.
    let p = Person { name: &name };

    // Fehler
    // assert_eq!(p.type_id(), TypeId::of::<Person>());

    let freda = Person { name: "Freda" };
    let fred = Person { name: "Fred" };

    // Ok
    assert_eq!(freda.type_id(), fred.type_id());
}
