use std::rc::{Rc, Weak};

struct Flaschenpost {
    str: Rc<String>,
}

pub fn main() {
    {
        // Referenzen mit clone erzeugen
        let rc = Rc::new(String::from("Hallo, Rust!"));

        // Jede Instanz erhält einen Klon
        {
            let post_1 = Flaschenpost { str: rc.clone() };
            {
                let post_2 = Flaschenpost { str: rc.clone() };
                {
                    let post_3 = Flaschenpost { str: rc.clone() };
                    // ...
                    println!("Ist ASCII: {}", post_3.str.is_ascii());
                }
                // ...
            }
            // ...
        }
        // ...
    }

    {
        // Veränderlicher Zugriff
        let mut rc = Rc::new(String::from("Hallo, Rust!"));

        // Achtung, Deref führt zur gleichnamigen str-Implementierung
        // let str = rc.get_mut(..);

        // Richtig, mit assoziierter Funktion
        let mut_rc: Option<&mut String> = Rc::get_mut(&mut rc);
        mut_rc.unwrap().make_ascii_uppercase();
        println!("{}", *rc);

        {
            let referenz = rc.clone();
            //
            let mut_rc = Rc::get_mut(&mut rc); // None
                                               // ...
            println!("Ref: {referenz:?}, mut: {mut_rc:?}");
        }

        {
            // make_mut
            // Keine geteilte Referenz aktiv
            let str = Rc::make_mut(&mut rc);

            // Schreibt ins Original
            *str = "Rusty!".into();
            println!("{}", *rc);

            let rc_klon = rc.clone();

            // Erstellt Klon, da rc_klon aktiv bleibt
            let str = Rc::make_mut(&mut rc);

            // Schreibt in den Klon
            *str = "Hallo, Rusty!".into();
            println!("Original: {}, lokale Kopie: {}", *rc_klon, *str);
            // Ausgabe: Original: Rusty!, lokale Kopie: Hallo, Rusty!
        }
    }

    {
        // Weak

        let rc = Rc::new(String::from("Hallo, Rust!"));

        let weak_ref: Weak<String> = Rc::downgrade(&rc.clone());

        drop(rc);

        let ressource: Option<Rc<String>> = weak_ref.upgrade();
        println!("Weak: {ressource:?}");
    }
}
