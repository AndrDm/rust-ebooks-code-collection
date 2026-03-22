#[derive(Debug)]
enum Geschlecht {
    M,
    W,
    D,
}

impl Drop for Geschlecht {
    fn drop(&mut self) {
        println!("drop von Geschlecht {self:?}")
    }
}

#[derive(Debug)]
struct Adresse;

impl Drop for Adresse {
    fn drop(&mut self) {
        println!("drop von Adresse")
    }
}

#[derive(Debug)]
struct Person {
    geschlecht: Geschlecht,
    adresse: Adresse,
}

impl Drop for Person {
    fn drop(&mut self) {
        println!("drop von Person")
    }
}

pub fn main() {
    {
        // drop nicht direkt aufrufen

        // let p = Person {
        //     geschlecht: Geschlecht::D,
        //     adresse: Adresse,
        // };

        // Fehler, kann nicht direkt aufgerufen werden
        // p.drop();
    }

    {
        // Automatische Freigabe am Ende des Blocks

        let _ = Person {
            geschlecht: Geschlecht::D,
            adresse: Adresse,
        };
    } // <-- Ende des lokalen Gültigkeitsbereichs

    {
        println!("Freigabe in Funktion");
        let geschlecht = Geschlecht::W;
        let adresse = Adresse;
    }

    {
        // drop veranlassen

        println!("std::mem::drop");

        let p = Person {
            geschlecht: Geschlecht::D,
            adresse: Adresse,
        };

        std::mem::drop(p); // <-- p wird bewegt und ungültig

        {
            println!("Eigenes Drop");

            let p_danach = Person {
                geschlecht: Geschlecht::D,
                adresse: Adresse,
            };

            let p = Person {
                geschlecht: Geschlecht::M,
                adresse: Adresse,
            };

            fn spezial_drop<T>(_d: T) {
                // _d wird freigegeben
            }

            spezial_drop(p_danach);
            println!("Nach dem drop");
        }
    }

    {
        // ManuallyDrop: drop in die eigenen Hände nehmen
        {
            println!("ManuallyDrop: Nicht aktiv");
            let a = Adresse;

            // <-- drop
        }

        // Ein veränderlicher Zeiger
        let p: *mut u8;

        {
            println!("ManuallyDrop: aktiv");
            let mut a = std::mem::ManuallyDrop::new(String::from("Hallo, Rust"));

            // Dereferenzierung, as_mut_ptr gehört zu String
            p = a.as_mut_ptr();
            // <-- Der Speicher hinter a wird nicht freigegeben
        }

        unsafe {
            let s = String::from_raw_parts(p, 11, 11);
            println!("{s}");

            // <-- Freigabe des ursprünglichen Speichers hier
        }
    }

    {
        // drop veranlassen

        println!("std::mem::forget");

        let p = Person {
            geschlecht: Geschlecht::D,
            adresse: Adresse,
        };

        std::mem::forget(p);
        // <-- p ist freigegeben, der Speicher aber noch besetzt

        // Fehler, schon bewegt
        // println!("{p:?}");
    }
}
