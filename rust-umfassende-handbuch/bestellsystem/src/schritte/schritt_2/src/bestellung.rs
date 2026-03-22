use crate::gast::Gast;
use crate::getraenk::GetraenkId;
use crate::speise::SpeiseId;

#[derive(PartialEq, Debug)]
pub struct Bestellung {
    getraenke: Vec<GetraenkId>,
    speisen: Vec<SpeiseId>,
}

impl Bestellung {
    pub fn new(
        speisen: Vec<SpeiseId>,
        getraenke: Vec<GetraenkId>,
    ) -> Bestellung {
        Bestellung { speisen, getraenke }
    }
}

pub struct BestellungBuilder {
    getraenke: Vec<GetraenkId>,
    speisen: Vec<SpeiseId>,
}

impl<'b> BestellungBuilder {
    pub fn new() -> BestellungBuilder {
        BestellungBuilder {
            getraenke: vec![],
            speisen: vec![],
        }
    }

    pub fn bestelle_getraenk(
        &mut self,
        getraenk: Option<GetraenkId>,
    ) -> &mut Self {
        if getraenk.is_none() {
            return self;
        }

        self.getraenke.push(getraenk.unwrap());

        self
    }

    pub fn bestelle_speise(&mut self, speise: Option<SpeiseId>) -> &mut Self {
        if speise.is_none() {
            return self;
        }

        self.speisen.push(speise.unwrap());

        self
    }

    pub fn bestellung_aufgeben(mut self) -> Bestellung {
        // Kostenloser Käsekuchen mit jeder Bestellung
        self.speisen.push(SpeiseId("Käsekuchen".to_string()));

        Bestellung::new(self.speisen, self.getraenke)
    }

    pub fn abbrechen(self) {
        // ... keine weiteren Schritte nötig
    }

    // pub fn waehle_extra<'a>(
    //     &mut self,
    //     gast: &Gast,
    //     option_1: &'a SpeiseId,
    //     option_2: &'a SpeiseId,
    // ) -> &'a SpeiseId {
    //     let option = gast.wuerfel();
    //     if option % 2 == 0 {
    //         option_1
    //     } else {
    //         option_2
    //     }
    // }

    pub fn waehle_extra(
        &mut self,
        gast: &Gast,
        option_1: &'b SpeiseId,
        option_2: &'b SpeiseId,
    ) -> &'b SpeiseId {
        if gast.hunger().0 >= 5 {
            option_1
        } else {
            option_2
        }
    }
}
