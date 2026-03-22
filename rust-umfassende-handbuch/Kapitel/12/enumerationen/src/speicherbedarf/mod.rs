enum OptionPerson {
    KeineAngaben, // Wie None
    // Alter, Name
    Person { alter: i32, name: String }, // Struktur-Variante
}

pub fn main() {
    let p1 = OptionPerson::KeineAngaben;
    let p1_size = std::mem::size_of_val(&p1);
    println!("p1 Größe: {}", p1_size); // p1 Größe: 32

    let p2 = OptionPerson::Person {
        alter: 42,
        name: "Freda".to_string(),
    };
    let p2_size = std::mem::size_of_val(&p2);
    println!("p2 Größe: {}", p2_size); // p2 Größe: 32

    // Variante mit Null-Pointer-Optimization
    {
        struct Person {
            alter: i32,
            name: String,
        }

        enum OptionPerson {
            KeineAngaben,
            Person(Box<Person>),
        }

        fn main() {
            let p: Option<Box<Person>> = None;
            let p_size = std::mem::size_of_val(&p);
            println!("p Größe: {}", p_size); // p Größe: 8
        }

        main();
    }
}
