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
