

pub fn main() {

    let str = "Hallo, Rust!";

    // Die Variable wird zum Alias für "str"
    let str_2: &str = str;

    // ...

    fn addiere(a: i32, b: i32) -> i32 {
        a + b
    }

    // Der Funktionszeiger ist ein Alias
    let addiere_alias = &addiere;


    // Die Typinformation vereinfachen und spezialisieren
    type Json = std::collections::HashMap<String, Box<dyn std::any::Any>>;

    // Mit dem Alias "Json" arbeiten
    let mut obj = Json::new();
    obj.insert("name".to_string(), Box::new("Rusty".to_string()));
    // ...
}