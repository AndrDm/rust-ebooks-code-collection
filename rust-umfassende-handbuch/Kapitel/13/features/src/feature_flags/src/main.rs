#![feature(let_chains)]

fn main() {
    let zahl_1 = Some(42);
    let zahl_2 = Some(21);

    // Achtung! Nur korrekt, wenn das instabile Feature
    // "let_chains" aktiviert worden ist.
    if let Some(a) = zahl_1 && let Some(b) = zahl_2 {
        println!("a: {a}, b: {b} ");
    }
}
