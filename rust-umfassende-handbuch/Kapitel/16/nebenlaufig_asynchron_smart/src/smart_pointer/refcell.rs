use std::cell::{Ref, RefCell, RefMut};

pub fn main() {
    let refcell = RefCell::new(String::new());

    // "replace" wie bei "Cell<T>"
    let vorher = refcell.replace_with(|_| "Hallo, Rust".into());
    // ...

    let str_1: Ref<String> = refcell.borrow();
    let str_2: Ref<String> = refcell.borrow();
    println!("{str_1}, {str_2}");

    let refcell = RefCell::new("Hallo, Rust".to_string());

    {
        // borrow_mut

        {
            let mut str_1: RefMut<String> = refcell.borrow_mut();

            // Ok
            str_1.make_ascii_uppercase();

            // Ok
            *str_1 = "Rusty".to_string();
        } // <- Lebenszeit von veränderlicher Referenz begrenzen

        println!("{}", refcell.borrow());
    }

    {
        // Vorsicht, wenn bereits exklusiv ausgeliehen

        // Kompiliert, aber ruft eine Panic hervor!
        let mut str_1: RefMut<String> = refcell.borrow_mut();
        // let str_2 = refcell.borrow();
        // println!("{str_1}, {str_2}");
    }

    {
        // try_borrow und try_borrow_mut
        let str_1: RefMut<String> = refcell.borrow_mut();
        // Ok, gibt "Err(_)" zurück
        let str_2 = refcell.try_borrow();
        // Ok, gibt "Err(_)" zurück
        let str_3 = refcell.try_borrow_mut();
        println!("{str_1:?},{str_2:?},{str_3:?}");
    }
}
