use prozedural::{nur_auf, PersonIdentifikation};

fn main() {
    #[nur_auf("macos")]
    #[derive(PersonIdentifikation)]
    struct Person {
        #[helper]
        alter: Option<i32>,
    };

    // Kontrolle: Ausgabe nur, wenn das
    // eigene Makro funktioniert, ansonsten Fehler!
    #[cfg(target_os = "macos")]
    {
        println!("{:?}", Person { alter: None });
        println!("{}", id_person());
    }
}
