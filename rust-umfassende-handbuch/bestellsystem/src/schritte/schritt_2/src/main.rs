use crate::bestellung::BestellungBuilder;
use crate::gast::Gast;
use crate::getraenk::GetraenkId;
use crate::restaurant::Restaurant;
use crate::speise::SpeiseId;
use crate::tisch::{Tisch, TischId};
use std::cell::RefCell;
use std::rc::Rc;

mod bestellung;
mod eigenschaften;
mod gast;
mod getraenk;
mod mitarbeiter;
mod restaurant;
mod speise;
mod speisekarte;
mod tisch;

fn main() {
    let zeiten = Restaurant::oeffnungszeiten();
    println!("{zeiten:?}");

    // Vollständig, aber dennoch ein Fehler!
    // let tisch = Tisch {
    //     id: TischId(1),
    //     belegung: None,
    //     stuehle: 0,
    //     gast: Rc::new(RefCell::new(None)),
    //     mitarbeiter: None,
    // };

    {
        // Fall &self
        let tisch = Tisch::new(1);
        let id_1 = tisch.id();

        // und noch einmal...
        let id_2 = tisch.id();
    }

    {
        // Fall self
        // let tisch = Tisch::new(1);
        // let id_1 = tisch.id();
        //
        // // Fehler! Die Instanz wurde oben
        // // aus "tisch" herausbewegt!
        // let id_2 = tisch.id();
    }

    let tisch = Tisch::new(1);
    let ist_frei = tisch.ist_frei(); // true

    // Fehler, kann nur aufgerufen werden, wenn in einer Box!
    // let ist_frei_sommer = tisch.ist_frei_sommer(false);

    //
    let sommer_tisch = Box::new(Tisch::new(21));

    // Ok
    let ist_frei = sommer_tisch.ist_frei();

    // Ok, in einer Box
    let ist_frei_sommer = sommer_tisch.ist_frei_sommer(true);

    // Bestellung-Builder
    let mut builder = BestellungBuilder::new();

    builder
        .bestelle_getraenk(Some(GetraenkId("Wasser".to_string())))
        // Oh, hier ist etwas schiefgelaufen
        .bestelle_getraenk(None)
        .bestelle_speise(Some(SpeiseId("Salat".to_string())))
        // Ups, vergessen, noch einen Espresso bitte!
        .bestelle_getraenk(Some(GetraenkId("Espresso".to_string())));

    let mut builder = BestellungBuilder::new();

    builder
        .bestelle_getraenk(Some(GetraenkId("Wasser".to_string())))
        .bestelle_speise(Some(SpeiseId("Salat".to_string())));

    let bestellung = builder.bestellung_aufgeben();
    println!("{bestellung:?}");
    // Oh, noch etwas vergessen
    // builder.bestelle_getraenk(Some(GetraenkId("Espresso".to_string())));

    let mut builder = BestellungBuilder::new();

    builder
        .bestelle_getraenk(Some(GetraenkId("Wasser".to_string())))
        .bestelle_speise(Some(SpeiseId("Salat".to_string())));
    let beigabe_1 = SpeiseId("Käsekuchen".to_string());
    let beigabe_2 = SpeiseId("Karottenkuchen".to_string());
    let gast = Gast::new();
    let kostenlose_zugabe = builder.waehle_extra(&gast, &beigabe_1, &beigabe_2);
    // clone, damit &SpeiseId zu SpeiseId wird
    builder.bestelle_speise(Some(kostenlose_zugabe.clone()));
    let bestellung = builder.bestellung_aufgeben();
    println!("{bestellung:?}");
}
