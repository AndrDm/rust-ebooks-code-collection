struct Person {
    name: String,
    // ...
}

impl AsRef<str> for Person {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

fn schreibe<T: AsRef<str>>(str: T) {
    println!("Geschrieben wird: {}", str.as_ref());
}

impl AsMut<String> for Person {
    fn as_mut(&mut self) -> &mut String {
        &mut self.name
    }
}

pub fn main() {
    schreibe("Ein String Slice");
    schreibe(String::from("Ein String"));

    let julia = Person {
        name: "Julia".to_string(),
        // ...,
    };
    schreibe(julia);

    // Veränderlich
    let mut julian = Person {
        name: "Julian".to_string(),
        // ...,
    };

    let name = julian.as_mut();
    name.replace_range(.., "Julian Junior");
}
