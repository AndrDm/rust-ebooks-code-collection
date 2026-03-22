
pub fn main() {
    {
        // Fehler, keine eindeutige Größe
        // struct Person(i32, Person);
    }

    {
        // Fehler, keine eindeutige Größe
        // struct Person {
        //     bester_freund: Option<Person>,
        // }

        {
            // Lösung mit Referenz
            struct Person<'a> {
                // Ok
                bester_freund: Option<&'a Person<'a>>,
            }
        }

        {
            // Lösung mit Box
            struct Person {
                // Ok
                bester_freund: Option<Box<Person>>,
            }
        }

        {
            // Lösung mit Vektor
            struct Person {
                // Ok
                bester_freund: Option<Vec<Person>>,
            }
        }

        {
            // Lösung mit Smart Pointer Rc

            use std::rc::Rc;
            use std::cell::RefCell;

            struct Person {
                bester_freund: Option<Rc<RefCell<Person>>>,
            }

            impl Person {
                fn schliesse_freundschaft(
                    &mut self,
                    freund: Rc<RefCell<Person>>
                ) {
                    self.bester_freund = Some(freund);
                }
            }

            let fred = Rc::new(
                RefCell::new(
                    Person { bester_freund: None }
                )
            );
            let barney = Rc::new(
                RefCell::new(
                    Person { bester_freund: None }
                )
            );

            let mut ref_fred = fred.borrow_mut();
            ref_fred.schliesse_freundschaft(barney.clone());

            let mut ref_barney = barney.borrow_mut();
            ref_barney.schliesse_freundschaft(fred.clone())
        }

        // let mut fred = Person { freunde: None };
        // let mut barney = Person { freunde: None };
        //
        // fred.freunde = []
    }
}