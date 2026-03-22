use crate::bestellung::Bestellung;
use crate::eigenschaften::Preis;
use crate::restaurant::Speisekarte;

#[derive(PartialEq, Debug)]
pub struct Rechnung {
    betrag: Preis,
}

impl Rechnung {
    pub fn from_bestellungen(
        bestellungen: &Vec<Bestellung>,
        speisekarte: &Speisekarte,
    ) -> Rechnung {
        let bestellungen_iter = bestellungen.iter();

        let rechnungsbetrag = bestellungen_iter.fold(0.0, |insgesamt, b| {
            insgesamt
                + b.speisen().iter().fold(0.0, |insgesamt, s| {
                    insgesamt + speisekarte.finde_speise(s).preis()
                })
                + b.getraenke().iter().fold(0.0, |insgesamt, g| {
                    insgesamt + speisekarte.finde_getraenk(g).preis()
                })
        });

        Rechnung {
            betrag: Preis(rechnungsbetrag),
        }
    }

    pub fn betrag(&self) -> Preis {
        self.betrag.clone()
    }
}
