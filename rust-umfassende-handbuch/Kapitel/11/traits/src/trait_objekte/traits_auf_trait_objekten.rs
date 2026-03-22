

trait Output {
    /// Komponenten erhalten eigenes Log-Tag
    fn debug_tag() where Self: Sized {
        println!("[Komponente] [Log]")
    }

    /// Soll jede Ausgabeform definieren
    fn write(&self, str: String);
}

struct Console { }

impl Output for Console {
    fn write(&self, str: String) {
        println!("Output Console: {str}");
    }
}

use std::fmt::{Debug, Display, Formatter, Result};

// Wir nutzen Console und Output aus den vorherigen Beispielen
impl Debug for dyn Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[Debug] Trait-Objekt von Console")
    }
}

impl Display for dyn Output {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[Display] Trait-Objekt von Console")
    }
}

pub fn main() {
    let output: &dyn Output = &Console {};

    // Nutzt Display
    println!("{output}");
    // Ausgabe: [Display] Trait-Objekt von Console

    // Nutzt Debug
    println!("{output:?}");
    // Ausgabe: [Debug] Trait-Objekt von Console

    // Fehler, Console implementiert nicht Display
    // println!("{}", Console {});
    // Fehler, &Console implementiert nicht Display
    // println!("{}", &Console {});
}