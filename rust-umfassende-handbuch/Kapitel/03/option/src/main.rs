fn main() {
    // Ein Array-Literal. Dazu mehr in Kapitel 6.2, "Array"
    let zahlen = [1, 2, 3, 4, 5];

    let element: Option<&i32> = zahlen.get(10);
    if element.is_some() {
        println!("Gefunden: {}", element.unwrap());
    }

    let element: Option<&i32> = zahlen.get(2);
    if element.is_some() {
        println!("Gefunden: {}", element.unwrap());
    }
}
