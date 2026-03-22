use std::borrow::Cow;

pub fn main() {
    let mut cow: Cow<str> = Cow::from("Hallo, Rust");

    // Über Deref den Wert auslesen und referenzieren
    let str: &str = &*cow;

    // Die Implementierung von "ToOwned" aufrufen
    let str: &mut String = cow.to_mut();
    // ...

    #[derive(Debug)]
    struct Person {
        alter: i32,
        name: String,
    };

    // "Person" muss als konkreter Datentyp
    // "ToOwned" implementieren
    impl ToOwned for Person {
        type Owned = Person;

        fn to_owned(&self) -> Self::Owned {
            Person {
                alter: self.alter,
                name: self.name.to_owned(),
            }
        }
    }

    let p: &Person = &Person { name: "Fred".into(), alter: 42 };

    let mut cow = Cow::Borrowed(p);
    // Fehler, ist geteilte Referenz in "cow"
    // cow.name = "Freda".into();

    let person: &mut Person = cow.to_mut();
    person.name = "Freda".into();

    println!("Person: {p:?}"); // Unverändert
    println!("Person: {person:?}"); // Klon in Cow verändert!
}
