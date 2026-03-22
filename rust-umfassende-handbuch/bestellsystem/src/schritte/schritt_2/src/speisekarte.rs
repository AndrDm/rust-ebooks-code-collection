use crate::getraenk::Getraenk;
use crate::speise::Speise;

#[derive(Clone)]
pub struct Speisekarte {
    speisen: Vec<Speise>,
    getraenke: Vec<Getraenk>,
}

// Später in eigene Dateien
// struct Speise;
// struct Getraenk;
