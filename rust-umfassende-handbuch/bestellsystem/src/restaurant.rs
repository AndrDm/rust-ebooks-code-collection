use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::gast::Gast;
use crate::mitarbeiter::{Mitarbeiter, MitarbeiterId};
use crate::tisch::{Tisch, TischId};

pub struct Restaurant {
    mitarbeiter: Vec<Rc<RefCell<Mitarbeiter>>>,
    tische: Vec<Rc<RefCell<Tisch>>>,
    zuordnung: HashMap<MitarbeiterId, Vec<TischId>>,
}

impl Restaurant {
    pub fn _oeffnungszeiten() -> HashMap<&'static str, &'static str> {
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

    pub fn new(
        tisch_zuordnung: HashMap<Mitarbeiter, Vec<Tisch>>,
    ) -> Restaurant {
        let mut alle_mitarbeiter = vec![];
        let mut alle_tische = vec![];

        let tisch_zuordnung = tisch_zuordnung
            .into_iter()
            .map(|e| {
                let mitarbeiter = e.0;
                let mitarbeiter_id = mitarbeiter.id();

                let tisch_ids: Vec<TischId> =
                    e.1.iter().map(|t| t.id()).collect();
                alle_tische
                    .extend(e.1.into_iter().map(|t| Rc::new(RefCell::new(t))));

                alle_mitarbeiter.push(Rc::new(RefCell::new(mitarbeiter)));
                (mitarbeiter_id, tisch_ids)
            })
            .collect();

        Restaurant {
            // tische,
            mitarbeiter: alle_mitarbeiter,
            tische: alle_tische,
            zuordnung: tisch_zuordnung,
        }
    }

    fn empfange_gast(&self, gast: Gast) -> GastZustand {
        let freier_tisch = self.hat_freien_tisch();
        if freier_tisch.is_none() {
            // Ist Gast bereit zu warten?
            return GastZustand::KeinFreierTisch;
        }

        let freier_tisch = freier_tisch
            .expect("Die Existenz von Tisch wird oben im Code geprüft.");

        self.zuweisung_tisch(gast, &freier_tisch.borrow());
        // let mitarbeiter_nach_zuordnung = self.zuordnung.find(|e| {
        //     e.1.iter()
        //         .find(|t| t.borrow().id() == freier_tisch.borrow().id())
        //         .is_some()
        // });

        let zuordnung = self.zuordnung.iter().find(|e| {
            e.1.iter()
                .find(|t| *t == &freier_tisch.borrow().id())
                .is_some()
        });

        match zuordnung {
            None => {
                // Diesem Tisch ist kein Mitarbeiter zugeordnet!
                return GastZustand::KeinMitarbeiterZugeordnet;
            }
            Some(mitarbeiter_zuordnung) => {
                let mitarbeiter_id = mitarbeiter_zuordnung.0;

                let mitarbeiter = self
                    .mitarbeiter
                    .iter()
                    .find(|m| &m.borrow().id() == mitarbeiter_id)
                    .expect("Schon oben geprüft");

                {
                    let mut tisch = freier_tisch.borrow_mut();
                    tisch.weise_mitarbeiter_zu(Some(mitarbeiter.to_owned()));
                }

                GastZustand::HatTischErhalten(freier_tisch)
            }
        }
    }

    fn tische(&self) -> Vec<Rc<RefCell<Tisch>>> {
        self.tische.clone()
        // self.zuordnung.iter().fold(vec![], |mut tische, zuordnung| {
        //     tische.extend(zuordnung.1.clone());
        //     tische
        // })
    }

    fn _mitarbeiter(&self) -> Vec<Rc<RefCell<Mitarbeiter>>> {
        self.mitarbeiter.clone()
    }

    fn hat_freien_tisch(&self) -> Option<Rc<RefCell<Tisch>>> {
        self.tische()
            .into_iter()
            .find(move |t| t.borrow().ist_frei())
    }

    fn zuweisung_tisch(&self, gast: Gast, tisch: &Tisch) {
        tisch.setze_gast(gast);
    }

    pub fn finde_mitarbeiter(
        &self,
        id: MitarbeiterId,
    ) -> Option<&Rc<RefCell<Mitarbeiter>>> {
        self.mitarbeiter.iter().find(|m| m.borrow().id() == id)
    }
}

#[derive(PartialEq, Debug)]
enum GastZustand {
    KeinFreierTisch,
    HatTischErhalten(Rc<RefCell<Tisch>>),
    KeinMitarbeiterZugeordnet,
}

#[cfg(test)]
mod restaurant_tests {
    use super::*;
    use crate::mitarbeiter::MitarbeiterId;
    use crate::speisekarte::Speisekarte;

    #[test]
    fn leeres_restaurant_keine_tische() {
        let restaurant = Restaurant::new(HashMap::new());

        let gast = Gast::new_zufall();

        let zustand = restaurant.empfange_gast(gast);
        assert_eq!(zustand, GastZustand::KeinFreierTisch);
    }

    #[test]
    fn gast_findet_tisch() {
        let freier_tisch = Tisch::new(0);
        let restaurant = Restaurant::new(HashMap::from_iter([(
            Mitarbeiter::new(0),
            vec![freier_tisch],
        )]));

        let gast = Gast::new_zufall();

        let zustand = restaurant.empfange_gast(gast);
        match zustand {
            GastZustand::HatTischErhalten(tisch) => {
                assert_eq!(tisch.borrow().id(), TischId::new(0));
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn ein_mitarbeiter_wird_zugewiesen() {
        let freier_tisch = Tisch::new(0);
        let mitarbeiter = Mitarbeiter::new(0);
        let restaurant = Restaurant::new(HashMap::from_iter([(
            mitarbeiter,
            vec![freier_tisch],
        )]));

        let gast = Gast::new_zufall();
        let zustand = restaurant.empfange_gast(gast);

        if let GastZustand::HatTischErhalten(tisch) = zustand {
            assert_eq!(
                tisch
                    .borrow()
                    .mitarbeiter()
                    .expect("Mitarbeiter sollte zugewiesen worden sein")
                    .borrow()
                    .id(),
                MitarbeiterId::new(0)
            );
        } else {
            assert!(false, "Der Gast hätte einen Tisch erhalten sollen");
        }
    }

    #[test]
    fn ein_mitarbeiter_nimmt_eine_bestellung_auf() {
        const MITARBEITER_ID: i32 = 0;
        let freier_tisch = Tisch::new(0);
        let mitarbeiter = Mitarbeiter::new(MITARBEITER_ID);
        let restaurant = Restaurant::new(HashMap::from_iter([(
            mitarbeiter,
            vec![freier_tisch],
        )]));

        let gast = Gast::new_zufall();
        let zustand = restaurant.empfange_gast(gast);
        if let GastZustand::HatTischErhalten(tisch) = zustand {
            let speisekarte = Speisekarte::empty();

            {
                let mitarbeiter = restaurant
                    .finde_mitarbeiter(MitarbeiterId::new(MITARBEITER_ID));
                mitarbeiter
                    .expect("ein Anderer Test prüft die Methode")
                    .borrow()
                    .nimm_bestellung_auf(tisch.clone(), speisekarte);
                // Borrow von mitarbeiter wieder freigeben
            }

            assert_eq!(tisch.borrow().bestellungen().len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn finde_mitarbeiter_fuer_id() {
        const MITARBEITER_ID: i32 = 0;
        let freier_tisch = Tisch::new(0);
        let mitarbeiter = Mitarbeiter::new(MITARBEITER_ID);
        let restaurant = Restaurant::new(HashMap::from_iter([(
            mitarbeiter,
            vec![freier_tisch],
        )]));

        let mitarbeiter =
            restaurant.finde_mitarbeiter(MitarbeiterId::new(MITARBEITER_ID));
        if let None = mitarbeiter {
            assert!(false, "Mitarbeiter existiert, wurde aber nicht gefunden!")
        }

        let mitarbeiter = restaurant.finde_mitarbeiter(MitarbeiterId::new(20));
        if let Some(_) = mitarbeiter {
            assert!(false, "Mitarbeiter existiert nicht, wurde aber gefunden!")
        }
    }
}
