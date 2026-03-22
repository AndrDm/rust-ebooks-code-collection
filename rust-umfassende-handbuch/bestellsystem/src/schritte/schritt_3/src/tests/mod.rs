pub mod fixture_speisekarte {
    use crate::{
        bestellung::{Getraenk, Speise},
        restaurant::Speisekarte,
    };

    pub fn new_speisekarte() -> Speisekarte {
        Speisekarte::new(
            vec![
                Speise::new(String::from("Pizza Salami"), 5, -1, 13.99),
                Speise::new("Salat".into(), 2, 0, 6.99),
            ],
            vec![
                Getraenk::new("Wasser gross".into(), 1, 5, 4.99),
                Getraenk::new(String::from("Wasser klein"), 1, 3, 3.99),
                Getraenk::new("Kaffee".into(), 1, 2, 4.59),
            ],
        )
    }
}
