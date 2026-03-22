use std::cell::Cell;

pub fn main() {

    {
        // Cell und Copy

        // Elementdatentyp wird inferiert
        let cell = Cell::new(0);

        // i32 ist Copy, daher verbleibt Wert in Cell
        let wert = cell.get();
        cell.set(wert + 1);       // 2
        cell.set(cell.get() + 1); // 3

        let vorher = cell.replace(4);

        println!("Zahl vorher: {}", cell.get());
    }

    {
        // Cell und !Copy
        let cell = Cell::new(String::from("Hallo, Rust"));

        // Ok, holt Wert, fügt gleichzeitig neu ein
        let str = cell.replace(Default::default());
        println!("{}", str);
        // Wie "replace(Default::default())"
        let str = cell.take();

        // Ok
        cell.set("Rusty".into());

        // Fehler, "get" nur mit T: Copy
        // let str = cell.get();
    }

    {
        // into_inner

        let cell = Cell::new(String::from("Hallo, Rust"));

        // Verbraucht die Cell
        let str = cell.into_inner();

        // Fehler, "cell" ist nicht mehr gültig
        // cell.set("".into());
    }

    {
        // get_mut, wenn Wert mut ist
        let mut cell = Cell::new("Hallo, Rust".to_string());
        let str = cell.get_mut();
        str.make_ascii_uppercase();
        println!("{str}");
    }

    {
        // Wandlung von veränderlicher Referenz zu Cell
        let mut str = String::new();
        let cell: &Cell<String> = Cell::from_mut(&mut str);

        cell.replace("Hallo, Rust".into());
        println!("Cell: {}", cell.take()); // Hallo, Rust
        println!("Cell: {}", cell.take()); // Leer
    }
}