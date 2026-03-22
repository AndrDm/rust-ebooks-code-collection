use prozedural::{einfuegen, sauber_einfuegen};
use quote::quote;

fn main() {
    {
        einfuegen!("Hallo Rust!");
        ausgabe = "Variable aus Makro".into();
        println!("{ausgabe}"); // Ok
    }
    {
        sauber_einfuegen!("Hallo Rust!");
        // Fehler, Hygiene isoliert die Variable
        // ausgabe = "Variable aus Makro".into();
        // println!("{ausgabe}"); // Ok
    }

    einfuegen!(quote! {});

    einfuegen! {
        #[derive(Debug)]
        struct Person {
            alter: i32,
            name: String
        }
    };
}
