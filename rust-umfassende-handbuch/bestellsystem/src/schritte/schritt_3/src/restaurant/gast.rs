use crate::{
    bestellung::{
        Bestellung, BestellungBuilder, BestellungLieferung, Getraenk, Speise,
    },
    eigenschaften::{Durst, Hunger},
    restaurant::Speisekarte,
};

#[derive(PartialEq, Debug, Clone)]
pub struct Gast {
    durst: Durst,
    hunger: Hunger,
}

impl Gast {
    pub const NICHT_DURSTIG: Durst = Durst(0);
    pub const NICHT_HUNGRIG: Hunger = Hunger(0);

    pub fn new(durst: Durst, hunger: Hunger) -> Gast {
        Gast { durst, hunger }
    }

    pub fn new_zufall() -> Gast {
        Gast {
            durst: Durst::new_zufall(),
            hunger: Hunger::new_zufall(),
        }
    }

    pub fn durst(&self) -> &Durst {
        &self.durst
    }

    pub fn hunger(&self) -> &Hunger {
        &self.hunger
    }

    pub fn gib_bestellung_auf(
        &self,
        bestellung: &mut BestellungBuilder,
        speisekarte: &Speisekarte,
    ) {
        let getraenk = self.waehle_getraenk(speisekarte);
        let speise = self.waehle_speise(speisekarte);

        bestellung
            .bestelle_getraenk(if let Some(getraenk) = getraenk {
                Some(getraenk.id())
            } else {
                None
            })
            .bestelle_speise(if let Some(speise) = speise {
                Some(speise.id())
            } else {
                None
            });
    }

    pub fn waehle_getraenk(
        &self,
        speisekarte: &Speisekarte,
    ) -> Option<Getraenk> {
        let durst_aktuell = self.durst;
        if durst_aktuell <= Gast::NICHT_DURSTIG {
            return None;
        }

        let mut kombinationen: Vec<_> = speisekarte
            .getraenke()
            .iter()
            .filter(move |k| k.durst().0 <= durst_aktuell.0)
            .map(|g| (g, durst_aktuell.0 - g.durst().0))
            .collect();

        kombinationen.sort_by(|k1, k2| k2.1.cmp(&k1.1));

        let gewuenschtes_getraenk = kombinationen.pop();
        if gewuenschtes_getraenk.is_none() {
            return None;
        }

        Some(gewuenschtes_getraenk.unwrap().0.clone())
    }

    pub fn waehle_speise(&self, speisekarte: &Speisekarte) -> Option<Speise> {
        let hunger_aktuell = self.hunger;
        if hunger_aktuell <= Gast::NICHT_HUNGRIG {
            return None;
        }

        let mut kombinationen: Vec<_> = speisekarte
            .speisen()
            .iter()
            .filter(move |k| k.hunger().0 <= hunger_aktuell.0)
            .map(|g| (g, hunger_aktuell.0 - g.hunger().0))
            .collect();

        kombinationen.sort_by(|k1, k2| k2.1.cmp(&k1.1));

        let gewuenschte_speise = kombinationen.pop();
        if gewuenschte_speise.is_none() {
            return None;
        }

        Some(gewuenschte_speise.unwrap().0.clone())
    }

    pub fn erhalte_bestellung(&mut self, lieferung: BestellungLieferung) {
        let mut durst_modifikation: Vec<_> =
            lieferung.getraenke().iter().map(|g| g.durst().0).collect();
        durst_modifikation
            .extend(lieferung.speisen().iter().map(|s| s.durst().0));

        let durst_modifikation = durst_modifikation
            .iter()
            .fold(0, |insgesamt, g| insgesamt + g);
        self.durst -= Durst(durst_modifikation);

        let mut hunger_modifikation: Vec<_> =
            lieferung.speisen().iter().map(|s| s.hunger().0).collect();
        hunger_modifikation
            .extend(lieferung.getraenke().iter().map(|g| g.hunger().0));
        let hunger_modifikation = hunger_modifikation
            .iter()
            .fold(0, |insgesamt, h| insgesamt + h);

        self.hunger -= Hunger(hunger_modifikation);
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Deref;

    use crate::{
        restaurant::{Mitarbeiter, Tisch},
        tests::fixture_speisekarte
    };

    use super::*;

    #[test]
    fn gast_erhaehlt_durst_und_hunger() {
        let gast = Gast::new_zufall();
        let durst = gast.durst();
        let hunger = gast.hunger();

        assert!(durst.0 <= 10);
        assert!(hunger.0 <= 10);
    }

    #[test]
    fn gast_waehlt_speise() {
        let speisekarte = Speisekarte::new(
            vec![
                Speise::new("Pizza Salami".into(), 5, -1, 0.0),
                Speise::new("Salat".into(), 2, 0, 0.0),
            ],
            vec![
                Getraenk::new("Wasser gross".into(), 1, 5, 0.0),
                Getraenk::new("Wasser klein".into(), 1, 3, 0.0),
                Getraenk::new("Kaffee".into(), 1, 2, 0.0),
            ],
        );
        let mut builder = BestellungBuilder::new();

        let gast = Gast::new(Durst(5), Hunger(2));
        gast.gib_bestellung_auf(&mut builder, &speisekarte);
        let bestellung = Bestellung::from_builder(builder);

        assert_eq!(bestellung.getraenke().len(), 1);
        assert_eq!(bestellung.getraenke()[0].0, "Wasser gross");
        assert_eq!(bestellung.speisen().len(), 1);
        assert_eq!(bestellung.speisen()[0].0, "Salat");
    }

    #[test]
    fn gast_hunger_und_durst_veraendern_sich() {
        let speisekarte = Speisekarte::new(
            vec![
                Speise::new("Pizza Salami".into(), 5, -1, 0.0),
                Speise::new("Salat".into(), 2, 0, 0.0),
            ],
            vec![
                Getraenk::new("Wasser gross".into(), 1, 5, 0.0),
                Getraenk::new("Wasser klein".into(), 1, 3, 0.0),
                Getraenk::new("Kaffee".into(), 1, 2, 0.0),
            ],
        );

        let gast = Gast::new(Durst(5), Hunger(2));
        let tisch = Tisch::new_cell(0);
        tisch.borrow_mut().setze_gast(gast);
        let mitarbeiter = Mitarbeiter::new(1);
        mitarbeiter.nimm_bestellung_auf(tisch.clone(), speisekarte);

        let tisch = tisch.borrow();
        if let Some(gast) = tisch.gast().borrow().deref() {
            assert_eq!(gast.durst.0, 0);
            // 1 von Getränk (Wasser gross) und 2 von Speise (Salat)
            assert_eq!(gast.hunger.0, -1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn gast_fordert_rechnung_an() {
        let gast = Gast::new_zufall();
        let tisch = Tisch::new_cell(0);
        tisch.borrow_mut().setze_gast(gast);
        let speisekarte = fixture_speisekarte::new_speisekarte();
        let mitarbeiter = Mitarbeiter::new(1);
        mitarbeiter.nimm_bestellung_auf(tisch.clone(), speisekarte);

        // gast.fordere_rechnung_an();

        // gast.bezahle_rechnung(rechnung);
    }
}
