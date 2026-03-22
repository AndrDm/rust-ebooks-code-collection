#![allow(unused)]

mod elternmodul {

    fn schreibe() {
        println!("Elternmodul: {}", module_path!());
    }

    fn schreibe_untermodul() {
        // self ist wird implizit angenommen und
        // kann daher eigentlich entfallen.
        self::untermodul::schreibe();
    }

    mod untermodul {
        // durch pub(super) nur im Elternmodul zugänglich
        pub(super) fn schreibe() {
            println!("Untermodul: {}", module_path!());
        }

        fn schreibe_elternmodul() {
            super::schreibe();
        }
    }
}

use bibliothek::oeffentliche_schnittstelle::log_api;

fn main() {

    // Ok
    log_api::info("Eine Logmeldung".to_string());

    // Fehler, das Modul ist nur innerhalb
    // des Crates "Bibliothek" zugänglich
    // log_api::log::flush_log();
    // log_api::log::write_log();

    mod elternmodul { // nicht kanonisch
        // ...
    }

    // Fehler, Sie können "elternmodul" nicht erreichen
    // use crate::main::???;

    // Ok
    use crate::elternmodul::*;

    // Pfad zu inhärenter Implementierung
    <crate::Person>::typ_name();

    let p = Person;
    // Pfad zur Trait-Implementierung
    <crate::Person as ToString>::to_string(&p);


}

struct Person;

impl Person {
    fn typ_name() -> &'static str {
        "Person"
    }
}

impl ToString for Person {
    fn to_string(&self) -> String {
        "Eine Person".to_string()
    }
}