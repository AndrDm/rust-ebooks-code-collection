// Nur primitive Datentypen, alles ist Copy
#[derive(Copy, Clone)]
struct Punkt {
    x: i32,
    y: i32,
}

// Person verwaltet einen String, daher nur Clone
// #[derive(Clone)]
struct Person {
    name: String,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        // wichtige Schritte ausführen ...

        // ...dann das Feld name klonen
        Self {
            name: self.name.clone(),
        }
    }
}

pub fn main() {
    let a = Punkt { x: 1, y: 2 };
    let b = a; // Implizite Kopie

    let fred = Person {
        name: "Fred".into(),
    };
    let fred_bewegt = fred; // Oh Oh, fred wurde bewegt

    let fred_kopie = fred_bewegt.clone(); // Kopie

    {
        // Implizite Kopie durch die Dereferenzierung
        let a: &Punkt = &Punkt { x: 1, y: 2 };
        let b: Punkt = *a; // implizite Kopie
    }

    let str = String::from("Hallo, Rust");

    let str1 = &str;
    let str2 = str1; // Kopie
    let str3 = str1; // Kopie
    println!("{str1}, {str2}, {str3}");

    let mut str = String::from("Hallo, Rust");
    let str4 = &mut str;
    let str5 = str4; // Kopie
    let str6 = str5; // Kopie
                     // println!("{str4}, {str5}, {str6}");

    // Der Versuch, Clone für &mut Person zu implementieren
    {
        struct Person;

        // impl Clone for &mut Person {
        //     fn clone(&self) -> Self {
        //         // Einfach, da unit-ähnlicher Typ
        //         Self
        //     }
        // }
    }
}
