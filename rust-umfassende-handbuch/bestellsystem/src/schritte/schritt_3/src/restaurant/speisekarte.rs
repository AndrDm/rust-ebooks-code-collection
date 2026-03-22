use crate::bestellung::{Getraenk, GetraenkId, Speise, SpeiseId};
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct Speisekarte {
    speisen: Vec<Speise>,
    getraenke: Vec<Getraenk>,
}

impl Speisekarte {
    pub fn empty() -> Speisekarte {
        Self::new(vec![], vec![])
    }

    pub fn new(speisen: Vec<Speise>, getraenke: Vec<Getraenk>) -> Speisekarte {
        Speisekarte { speisen, getraenke }
    }

    pub fn getraenke(&self) -> &Vec<Getraenk> {
        &self.getraenke
    }
    pub fn speisen(&self) -> &Vec<Speise> {
        &self.speisen
    }

    pub fn finde_speise(&self, id: &SpeiseId) -> Speise {
        self.speisen()
            .iter()
            .find(|sp| &sp.id() == id)
            .expect("Unbekannte SpeiseIds kann es derzeit nicht geben")
            .clone()
    }

    pub fn finde_getraenk(&self, id: &GetraenkId) -> Getraenk {
        self.getraenke()
            .iter()
            .find(|sp| &sp.id() == id)
            .expect("Unbekannte GetraenkIds kann es derzeit nicht geben")
            .clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speisekarte_aus_json() {
        let speisekarte_json = r#"
            {
                "speisen": [
                    {
                        "name": "Pizza",
                        "hunger": 5,
                        "durst": 0,
                        "preis": 13.99
                    }
                ],
                "getraenke": [
                    {
                        "name": "Wasser",
                        "hunger": 1,
                        "durst": 3,
                        "preis": 3.99
                    }
                ]
            }
        "#;

        let speisekarte: Speisekarte = serde_json::from_str(speisekarte_json)
            .expect("Hat sich das Format geändert?");

        assert_eq!(speisekarte.speisen.len(), 1);
        assert_eq!(speisekarte.getraenke.len(), 1);
    }
}
