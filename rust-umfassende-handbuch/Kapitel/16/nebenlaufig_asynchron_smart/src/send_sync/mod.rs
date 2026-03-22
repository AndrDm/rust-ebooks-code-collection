#![cfg(feature = "thread_send_sync")]

use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

struct Person {
    // ...
    _nicht_send: PhantomData<Rc<bool>>,
    // ...
    alter: Cell<Option<i32>>,
}

impl Person {
    pub fn new() -> Person {
        Person {
            _nicht_send: PhantomData,
            alter: Cell::new(None),
        }
    }

    fn neues_alter(&self, alter: i32) {
        self.alter.replace(Some(alter));
    }
}

fn sender<T>(t: T) where T: Send {}

pub fn main() {

    {
        use std::thread;

        let str = "Hallo, Rust";
        // Ok! Nur &'static str Referenzen
        thread::scope(|scope| {
            scope.spawn(|| println!("{str}"));
            scope.spawn(|| println!("{str}"));
            scope.spawn(|| println!("{str}"));
        });

        let str = String::from(str);
        // "Leak" überführt in die statische Lebenszeit
        let mut str: &mut str = str.leak();

        // So nicht möglich! Gleichzeitig eine
        // veränderliche und zwei geteilte Referenzen aktiv
        // thread::scope(|scope| {
        //     scope.spawn(|| println!("{str}"));
        //     scope.spawn(|| {
        //         // Ok, wenn nur dieser Thread aktiv wäre
        //         str.make_ascii_uppercase();
        //         println!("{str}");
        //     });
        //     scope.spawn(|| println!("{str}"));
        // });
    }

    let p = Person::new();
    // Fehler, Person ist nicht Send!
    // sender(p);

    {
        unsafe impl Send for Person {}
        unsafe impl Sync for Person {}

        // Ok, lokale Implementierung des Traits
        let p = Person::new();
        sender(p);
    }
}