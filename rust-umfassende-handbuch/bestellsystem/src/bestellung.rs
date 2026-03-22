use crate::getraenk::{Getraenk, GetraenkId};
use crate::speise::{Speise, SpeiseId};
use crate::speisekarte::Speisekarte;

#[derive(PartialEq, Debug)]
pub struct Bestellung {
    getraenke: Vec<GetraenkId>,
    speisen: Vec<SpeiseId>,
}

impl Bestellung {
    pub(super) fn new(
        speisen: Vec<SpeiseId>,
        getraenke: Vec<GetraenkId>,
    ) -> Bestellung {
        Bestellung { speisen, getraenke }
    }

    pub fn from_builder(builder: BestellungBuilder) -> Bestellung {
        builder.bestellung_aufgeben()
    }

    pub fn getraenke(&self) -> &Vec<GetraenkId> {
        &self.getraenke
    }
    pub fn speisen(&self) -> &Vec<SpeiseId> {
        &self.speisen
    }
}

pub struct BestellungBuilder {
    getraenke: Vec<GetraenkId>,
    speisen: Vec<SpeiseId>,
}

impl BestellungBuilder {
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

    pub fn bestellung_aufgeben(self) -> Bestellung {
        Bestellung::new(self.speisen, self.getraenke)
    }
}

pub struct BestellungLieferung {
    getraenke: Vec<Getraenk>,
    speisen: Vec<Speise>,
}

impl BestellungLieferung {
    fn new(
        getraenke: Vec<Getraenk>,
        speisen: Vec<Speise>,
    ) -> BestellungLieferung {
        BestellungLieferung { getraenke, speisen }
    }

    pub fn from_bestellung(
        bestellung: &Bestellung,
        speisekarte: &Speisekarte,
    ) -> BestellungLieferung {
        let speisen: Vec<Speise> = bestellung
            .speisen()
            .iter()
            .map(|s| speisekarte.finde_speise(s))
            .collect();

        let getraenke: Vec<Getraenk> = bestellung
            .getraenke()
            .iter()
            .map(|g| speisekarte.finde_getraenk(g))
            .collect();

        BestellungLieferung::new(getraenke, speisen)
    }

    pub fn getraenke(&self) -> &Vec<Getraenk> {
        &self.getraenke
    }
    pub fn speisen(&self) -> &Vec<Speise> {
        &self.speisen
    }
}
