use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use super::{Speisekarte, Tisch, TischId};
use crate::bestellung::{BestellungBuilder, BestellungLieferung, Rechnung};
#[cfg(test)]
use crate::tests::fixture_speisekarte;

/// Ein Mitarbeiter des Restaurants.
/// Die Komponente wird einem [Tisch] zugewiesen und kümmert
/// sich um etwaige [Gäste](crate::restaurant::Gast).
#[doc(alias = "Angestellter")]
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Mitarbeiter {
    id: MitarbeiterId,
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct MitarbeiterId(i32);

impl MitarbeiterId {
    pub fn new(id: i32) -> MitarbeiterId {
        MitarbeiterId(id)
    }
}

impl Mitarbeiter {
    pub fn new(id: i32) -> Mitarbeiter {
        Mitarbeiter {
            id: MitarbeiterId::new(id),
        }
    }

    pub fn nimm_bestellung_auf(
        &self,
        tisch: Rc<RefCell<Tisch>>,
        speisekarte: Speisekarte,
    ) {
        let gast = tisch.borrow().gast();
        let mut gast = gast.borrow_mut();

        match gast.deref_mut() {
            None => {
                // Ein Mitarbeiter versucht, eine Bestellung für einen leeren Tisch
                // einzutragen.
                return;
            }
            Some(gast) => {
                // Bestellung aufnehmen
                let mut bestellung_builder = BestellungBuilder::new();
                gast.gib_bestellung_auf(&mut bestellung_builder, &speisekarte);

                let bestellung = bestellung_builder.bestellung_aufgeben();

                let lieferung = BestellungLieferung::from_bestellung(
                    &bestellung,
                    &speisekarte,
                );

                tisch.borrow_mut().registriere_bestellung(bestellung);

                gast.erhalte_bestellung(lieferung);
            }
        }
    }

    pub fn id(&self) -> MitarbeiterId {
        self.id
    }

    /// Der Mitarbeiter steht am Kassensystem. Er startet einen Rundgang,
    /// bei dem er alle Tische, die ihm zugewiesen wurden, überprüft. Dabei
    /// sieht er nach, ob ein Gast einen Wunsch hat oder andere Aufgaben am Tisch
    /// zu erledigen sind. Etwa reinigen oder Geschirr abräumen.
    pub fn starte_runde(&self, _tische: Vec<TischId>) {
        // thread::
    }

    pub fn erstelle_rechnung(
        &self,
        tisch: Rc<RefCell<Tisch>>,
        speisekarte: &Speisekarte,
    ) -> Rechnung {
        let tisch = tisch.borrow();

        let bestellungen = tisch.bestellungen();

        Rechnung::from_bestellungen(bestellungen, speisekarte)
    }
}

#[cfg(test)]
mod tests {
    use crate::bestellung::{GetraenkId, SpeiseId};

    use super::*;

    #[test]
    fn mitarbeiter_zyklus() {
        let mitarbeiter = Mitarbeiter::new(1);
        let tische =
            Vec::from_iter([1, 5, 7].map(|n| TischId::new(n)).to_vec());
        mitarbeiter.starte_runde(tische);
    }

    #[test]
    fn mitarbeiter_erstellt_rechnung() {
        let mitarbeiter = Mitarbeiter::new(1);

        let tisch = Tisch::new_cell(1);
        let speisekarte = fixture_speisekarte::new_speisekarte();

        let mut bestellung_builder = BestellungBuilder::new();
        bestellung_builder
            .bestelle_speise(Some(SpeiseId("Pizza Salami".into())))
            .bestelle_getraenk(Some(GetraenkId("Wasser klein".into())));
        let bestellung_1 = bestellung_builder.bestellung_aufgeben();
        tisch.borrow_mut().registriere_bestellung(bestellung_1);

        let rechnung =
            mitarbeiter.erstelle_rechnung(tisch.clone(), &speisekarte);

        tisch.borrow_mut().erhalte_rechnung(rechnung);

        assert_eq!(
            tisch
                .borrow()
                .rechnung()
                .as_ref()
                .expect("Rechnung oben übergeben")
                .betrag()
                .0,
            17.98
        );
    }
}
