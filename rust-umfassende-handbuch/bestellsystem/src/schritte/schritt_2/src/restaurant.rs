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

impl Restaurant {
    pub fn oeffnungszeiten() -> HashMap<&'static str, &'static str> {
        if Restaurant::ist_sommer() {
            // Im Sommer früher öffnen
            return HashMap::from_iter([
                ("Mo", "12:00 - 24:00"),
                ("Di", "12:00 - 24:00"),
                ("Mi", "12:00 - 24:00"),
                ("Do", "12:00 - 24:00"),
                ("Fr", "12:00 - 24:00"),
                ("Sa", "12:00 - 24:00"),
                ("So", "Geschlossen"),
            ]);
        }

        HashMap::from_iter([
            ("Mo", "16:00 - 24:00"),
            ("Di", "16:00 - 24:00"),
            ("Mi", "16:00 - 24:00"),
            ("Do", "16:00 - 24:00"),
            ("Fr", "16:00 - 24:00"),
            ("Sa", "12:00 - 24:00"),
            ("So", "Geschlossen"),
        ])
    }

    pub fn ist_sommer() -> bool {
        // Kalenderdaten abfragen ...
        true
    }
}
