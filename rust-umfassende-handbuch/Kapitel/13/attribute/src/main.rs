// Gültig im ganzen Modulbaum
#![allow(dead_code)]
#![allow(unused_variables)]
// #![forbid(unused_must_use)]

fn schreibe_hilfe() {
    #![allow(unused_imports)]
    use std::collections::HashMap;
}

// ...
struct Person;

impl Person {
    #![deny(unused_imports)]
    // ...

    fn new() -> Person {
        // Fehler, siehe deny
        // use std::collections::HashMap;
        // ...
        todo!()
    }

    fn schreibe_daten() {
        // Fehler, siehe deny
        // use std::collections::HashMap;
        // ...
    }
}

extern "C" {
    // Diesen Block nur kompilieren, wenn
    // das Target Windows ist
    #![cfg(windows)]

    fn ExterneSchnittstelleA();
    fn ExterneSchnittstelleB();
}

fn main() {
    // Ok, Regel ist auf "allow"
    let a: i32;

    #[warn(unused_variables)]
    mod sicherer_bereich {
        fn berechne_zahl() {
            // Warnung!
            let anfang: i32;
        }
    }

    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone)]
    enum personen_daten {
        komplette_angabe,
        teilweise,
        keine_angabe,
    }

    #[derive(Copy, Clone)]
    struct A;

    // #[deprecated]
    // #[deprecated = "Bitte nicht benutzen!"]
    #[deprecated(since = "0.1.1", note = "Bitte nicht benutzen")]
    struct B;

    let b = B;

    #[must_use]
    fn berechne() -> i32 {
        todo!()
    }

    // Warnung, Rückgabe nicht verwendet
    berechne();
}

#[inline]
fn verarbeitete_person_response(p: Person) {
    // ...
}

#[inline(always)]
fn person_unvollstaendig(p: Person) {}

#[inline(never)]
fn person_nicht_gefunden(p: Person) {}

#[cold]
fn zeige_regsitrierungs_formular() {}
