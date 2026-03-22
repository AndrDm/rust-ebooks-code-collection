use std::cell::RefCell;
use std::rc::Rc;

use crate::gast::Gast;
use crate::mitarbeiter::Mitarbeiter;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TischId(i32);

#[derive(PartialEq, Debug)]
pub struct Tisch {
    id: TischId,
    belegung: Option<i32>,
    stuehle: i32,
    gast: Rc<RefCell<Option<Gast>>>,
    mitarbeiter: Option<Rc<RefCell<Mitarbeiter>>>,
}

impl Tisch {
    pub fn new(id: i32) -> Tisch {
        Tisch {
            id: TischId(id),
            belegung: None,
            stuehle: 4,
            gast: Default::default(),
            mitarbeiter: Default::default(),
        }
    }

    pub fn id(&self) -> TischId {
        self.id
    }

    // Achtung, wird die Instanz verbrauchen
    // pub fn id(self) -> TischId {
    //     self.id
    // }

    pub fn ist_frei(&self) -> bool {
        self.belegung.is_none()
    }

    pub fn ist_frei_sommer(self: &Box<Self>, ist_sommer: bool) -> bool {
        if !ist_sommer {
            return false;
        }

        self.ist_frei()
    }

    pub fn setze_gast(&self, gast: Gast) {
        self.gast.replace(Some(gast));
    }

    pub fn gast(&self) -> Rc<RefCell<Option<Gast>>> {
        self.gast.clone()
    }
}
