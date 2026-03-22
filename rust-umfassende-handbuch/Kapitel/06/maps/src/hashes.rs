use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Eq, PartialEq, /*Hash*/)]
struct Person {
    name: String,
}

impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

struct Buch {
    titel: String,
}

pub fn main() {
    let mut map = HashMap::new();

    let fred = Person { name: "Fred".to_string() };
    // Fehler, Person implementiert nicht die Traits
    // Eq und Hash
    map.insert(fred, vec![
        Buch { titel: "Alice im Wunderland".to_string() }
    ]);
}