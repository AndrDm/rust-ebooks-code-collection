// restaurant.rs
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::mitarbeiter::{Mitarbeiter, MitarbeiterId};
use crate::tisch::{Tisch, TischId};

pub struct Restaurant {
    mitarbeiter: Vec<Rc<RefCell<Mitarbeiter>>>,
    tische: Vec<Rc<RefCell<Tisch>>>,
    zuordnung: HashMap<MitarbeiterId, Vec<TischId>>,
}
