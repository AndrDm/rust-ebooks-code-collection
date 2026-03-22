use std::cell::RefCell;
use std::rc::Rc;

use crate::bestellung::Bestellung;
use crate::gast::Gast;
use crate::mitarbeiter::Mitarbeiter;
use crate::rechnung::Rechnung;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TischId(i32);

impl TischId {
    pub fn new(id: i32) -> TischId {
        TischId(id)
    }
}

#[derive(PartialEq, Debug)]
pub struct Tisch {
    id: TischId,
    belegung: Option<i32>,
    stuehle: i32,
    gast: Rc<RefCell<Option<Gast>>>,
    mitarbeiter: Option<Rc<RefCell<Mitarbeiter>>>,
    bestellungen: Vec<Bestellung>,
    rechnung: Option<Rechnung>,
}

impl Tisch {
    pub fn new(id: i32) -> Tisch {
        Tisch {
            id: TischId(id),
            belegung: None,
            stuehle: 4,
            gast: Default::default(),
            mitarbeiter: Default::default(),
            bestellungen: vec![],
            rechnung: None,
        }
    }

    pub fn new_cell(id: i32) -> Rc<RefCell<Tisch>> {
        Rc::new(RefCell::new(Self::new(id)))
    }

    pub fn id(&self) -> TischId {
        self.id
    }

    pub fn mitarbeiter(&self) -> Option<Rc<RefCell<Mitarbeiter>>> {
        self.mitarbeiter.clone()
    }

    pub fn bestellungen(&self) -> &Vec<Bestellung> {
        &self.bestellungen
    }

    pub fn weise_mitarbeiter_zu(
        &mut self,
        mitarbeiter: Option<Rc<RefCell<Mitarbeiter>>>,
    ) {
        self.mitarbeiter = mitarbeiter;
    }

    pub fn ist_frei(&self) -> bool {
        self.belegung.is_none()
    }

    pub fn setze_gast(&self, gast: Gast) {
        self.gast.replace(Some(gast));
    }

    pub fn gast(&self) -> Rc<RefCell<Option<Gast>>> {
        self.gast.clone()
    }

    pub fn rechnung(&self) -> &Option<Rechnung> {
        &self.rechnung
    }

    pub fn _entferne_stuhl(&mut self) {
        self.stuehle -= 1;
    }
    pub fn _nimm_stuhl_hinzu(&mut self) {
        self.stuehle += 1;
    }

    pub fn registriere_bestellung(&mut self, bestellung: Bestellung) {
        self.bestellungen.push(bestellung);
    }

    pub fn erhalte_rechnung(&mut self, rechnung: Rechnung) {
        self.rechnung = Some(rechnung);
    }
}
