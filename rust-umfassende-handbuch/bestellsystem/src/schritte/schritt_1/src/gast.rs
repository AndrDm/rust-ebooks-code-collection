use crate::eigenschaften::{Durst, Hunger};

#[derive(PartialEq, Debug, Clone)]
pub struct Gast {
    durst: Durst,
    hunger: Hunger,
}