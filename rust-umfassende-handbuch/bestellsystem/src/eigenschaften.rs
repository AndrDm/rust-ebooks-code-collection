use rand::Rng;
use serde::Deserialize;
use std::ops::{Add, Sub, SubAssign};

#[derive(PartialEq, Debug, Copy, Clone, Deserialize, PartialOrd)]
pub struct Durst(pub i32);
#[derive(PartialEq, Debug, Copy, Clone, Deserialize, PartialOrd)]
pub struct Hunger(pub i32);

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

impl Add for Durst {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Durst {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Durst {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Hunger {
    pub fn new_zufall() -> Hunger {
        let hunger = zahl_zwischen_1_und_10();
        Hunger(hunger)
    }
}

impl Add for Hunger {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Hunger {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Hunger {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Preis(pub f64);

impl Add<f64> for Preis {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        self.0 + rhs
    }
}

impl Add<Preis> for f64 {
    type Output = f64;

    fn add(self, rhs: Preis) -> Self::Output {
        self + rhs.0
    }
}

impl Add<&Preis> for f64 {
    type Output = f64;

    fn add(self, rhs: &Preis) -> Self::Output {
        self + rhs.0
    }
}
