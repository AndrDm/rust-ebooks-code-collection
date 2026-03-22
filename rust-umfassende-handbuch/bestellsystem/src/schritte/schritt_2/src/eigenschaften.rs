use rand::Rng;

#[derive(PartialEq, Debug, Copy, Clone, PartialOrd)]
pub struct Durst(pub i32);

fn zahl_zwischen_1_und_10() -> i32 {
    let mut generator = rand::thread_rng();
    generator.gen_range(0..=10)
}

impl Durst {
    pub fn new_zufall() -> Durst {
        let durst = zahl_zwischen_1_und_10();
        Durst(durst)
    }
}

#[derive(PartialEq, Debug, Copy, Clone, PartialOrd)]
pub struct Hunger(pub i32);

impl Hunger {
    pub fn new_zufall() -> Hunger {
        let hunger = zahl_zwischen_1_und_10();
        Hunger(hunger)
    }
}
