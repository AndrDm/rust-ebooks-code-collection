use crate::speisekarte::Speisekarte;
use crate::tisch::Tisch;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Mitarbeiter {
    id: MitarbeiterId,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct MitarbeiterId(i32);

impl Mitarbeiter {
    pub fn nimm_bestellung_auf(
        &self,
        tisch: Rc<RefCell<Tisch>>,
        speisekarte: Speisekarte,
    ) {
        let gast: Rc<RefCell<_>> = tisch.borrow().gast();
        let mut gast = gast.borrow_mut();

        match gast.deref_mut() {
            None => {
                // Ein Mitarbeiter versucht, eine Bestellung
                // für einen leeren Tisch einzutragen.
                return;
            }
            Some(gast) => {
                // Bestellung aufnehmen ...
            }
        }
    }
}
