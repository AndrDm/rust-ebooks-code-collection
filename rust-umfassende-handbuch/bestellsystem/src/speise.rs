use crate::eigenschaften::{Durst, Hunger, Preis};
use serde::Deserialize;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Speise {
    name: SpeiseId,
    hunger: Hunger,
    durst: Durst,
    preis: Preis,
}

impl Speise {
    pub fn new(name: String, hunger: i32, durst: i32, preis: f64) -> Speise {
        Speise {
            name: SpeiseId(name),
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

    pub fn id(&self) -> SpeiseId {
        self.name.clone()
    }

    pub fn preis(&self) -> &Preis {
        &self.preis
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct SpeiseId(pub String);
