use crate::eigenschaften::{Durst, Hunger};

#[derive(PartialEq, Debug, Clone)]
pub struct Gast {
    durst: Durst,
    hunger: Hunger,
}

impl Gast {
    pub fn new() -> Gast {
        Gast {
            durst: Durst::new_zufall(),
            hunger: Hunger::new_zufall(),
        }
    }

    pub fn hunger(&self) -> &Hunger {
        &self.hunger
    }
}
