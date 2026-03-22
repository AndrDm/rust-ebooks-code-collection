use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
struct Adresse;

#[derive(Debug)]
struct Person {
    name: String,
    adresse: Adresse,
    alter: i32,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Nur den Hash des Strings einfügen
        self.name.hash(state)
    }
}

impl PartialEq<Self> for Person {
    fn eq(&self, other: &Self) -> bool {
        // Gleich, wenn die Namen übereinstimmen
        self.name == other.name
    }
}

impl Eq for Person {}

impl Borrow<str> for Person {
    fn borrow(&self) -> &str {
        &self.name
    }
}

// Bedingt durch BorrowMut<String>
impl Borrow<String> for Person {
    fn borrow(&self) -> &String {
        &self.name
    }
}

impl BorrowMut<String> for Person {
    fn borrow_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub fn main() {
    {
        // Beispiel: Geteilte Identität von String und &str
        let str = "Hallo, Rust";
        let string = String::from(str);
        let mut string2 = String::from(str);
        assert_eq!(string, string2); // true
        assert_eq!(string, str); // true

        string2.push_str("!");
        // assert_eq!(string, string2); // false
    }

    {
        // Als Key einer HashMap
        let barney = Person {
            name: "Barney".to_string(),
            adresse: Adresse,
            alter: 54,
        };

        let mut map = HashMap::new();
        map.insert(
            barney,
            vec![Person {
                name: "Fred".to_string(),
                adresse: Adresse,
                alter: 51,
            }],
        );

        let freunde_von_barney = &map["Barney"];
        println!("Freunde: {freunde_von_barney:?}");
    }

    {
        // Umgang mit borrow
        let mut p = Person {
            name: "Barney".to_string(),
            adresse: Adresse,
            alter: 54,
        };

        let name: &mut String = p.borrow_mut();
        name.replace_range(.., "Fred");
        // ...
        println!("{p:?}");

        // Generischer Code erwartet Borrow<String>
        fn borrow_typ<T: Borrow<String>>(ausleihen: &T) {
            let wert = ausleihen.borrow();
            // ...
        }

        // Ok, Person implementiert Borrow<String>
        borrow_typ(&p);
    }
}
