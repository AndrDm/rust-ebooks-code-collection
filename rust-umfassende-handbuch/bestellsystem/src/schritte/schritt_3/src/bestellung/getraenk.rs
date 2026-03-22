use crate::eigenschaften::{Durst, Hunger, Preis};
use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Getraenk {
    name: GetraenkId,
    hunger: Hunger,
    durst: Durst,
    preis: Preis,
}

impl Getraenk {
    pub fn new(name: String, hunger: i32, durst: i32, preis: f64) -> Getraenk {
        Getraenk {
            name: GetraenkId(name),
            hunger: Hunger(hunger),
            durst: Durst(durst),
            preis: Preis(preis),
        }
    }

    pub fn hunger(&self) -> Hunger {
        self.hunger
    }
    pub fn durst(&self) -> Durst {
        self.durst
    }

    pub fn id(&self) -> GetraenkId {
        self.name.clone()
    }

    pub fn preis(&self) -> &Preis {
        &self.preis
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct GetraenkId(pub String);
