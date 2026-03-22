use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseFloatError;
use std::process::ExitCode;

#[derive(Debug)]
enum Fehlmessung {
    Hardware(HardwareFehler),
    GrafischeOberflaeche(std::num::ParseFloatError),
}

impl Error for Fehlmessung {}

impl Display for Fehlmessung {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Fehlmessung::Hardware(h) => f.write_fmt(format_args!("{h}")),
            Fehlmessung::GrafischeOberflaeche(g) => f.write_fmt(format_args!("{g}")),
        }
    }
}

impl From<HardwareFehler> for Fehlmessung {
    fn from(value: HardwareFehler) -> Self {
        Self::Hardware(value)
    }
}

impl From<std::num::ParseFloatError> for Fehlmessung {
    fn from(value: std::num::ParseFloatError) -> Self {
        Self::GrafischeOberflaeche(value)
    }
}

#[derive(Debug)]
struct HardwareFehler(u32);

impl Display for HardwareFehler {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Hardwarefehler: {}", self.0))
    }
}
impl Error for HardwareFehler {}

impl From<u32> for HardwareFehler {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

struct Gewicht(f64);

// Weil Schnittstelle, besser den Fehler als generischen Fehler liefern
// pub fn zielgewicht_erreicht() -> Result<bool, Box<dyn Error + Sync + Send + 'static>> {
fn zielgewicht_erreicht() -> Result<bool, Fehlmessung> {
    // auf die Hardware zugreifen
    let gewicht = lese_aus_speicher()?;

    // mit Eingabe aus UI vergleichen
    let zielgewicht = frage_nutzer_nach_zielgewicht()?;

    // mit gewicht und zielgewicht weiterarbeiten
    // ...
    Ok(gewicht <= zielgewicht.0)
}

fn lese_aus_speicher() -> Result<f64, HardwareFehler> {
    // Oh Oh, ein Fehler ist aufgetreten
    let fehlerspeicher = 76;

    Err(HardwareFehler::from(fehlerspeicher))
}

fn frage_nutzer_nach_zielgewicht() -> Result<Gewicht, ParseFloatError> {
    use std::str::FromStr;

    let eingabe = "182,3";
    let numerisch = f64::from_str(eingabe)?;

    Ok(Gewicht(numerisch))
}

pub fn main() -> Result<ExitCode, Box<dyn Error>> {
    let zielgewicht_erreicht = zielgewicht_erreicht()?;

    return Ok(ExitCode::SUCCESS);
}
