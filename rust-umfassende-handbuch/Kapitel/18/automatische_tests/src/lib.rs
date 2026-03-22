use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct DatenbankFehler;

impl Display for DatenbankFehler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for DatenbankFehler {}

// ...

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::{Debug, Display}};

    use super::DatenbankFehler;

    fn fake_datenbank() -> Result<String, Box<dyn Error>> {
        Err(Box::new(DatenbankFehler))
    }

    #[test]
    fn lese_konfiguration_aus_datenbank() -> Result<(), Box<dyn Error>> {
        // Test schlägt automatisch fehl, wenn Err(_)
        // let datenbank = fake_datenbank()?;

        // ...
        Ok(())
    }

    #[test]
    #[ignore]
    fn konfiguration_speichern() -> Result<(), Box<dyn Error>> {
        // Sorgen aber dafür,
        // dass Sie diesen Test nicht vergessen.
        Ok(())
    }

    #[test]
    #[ignore = "Die Anforderungen sind noch nicht klar!"]
    fn konfiguration_anwenden() {}
}