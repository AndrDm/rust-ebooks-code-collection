
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

// impl std::fmt::Display for dyn Output {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

pub fn main() {

    {
        // Ein Trait von Trait-Objekten ausschließen
        trait Form where Self: Sized {

        }

        struct Punkt {}
        impl Form for Punkt {}

        // Fehler, Form kann nicht zu einem Trait-Objekt werden
        // let p: &dyn Form = &Punkt {};
    }





    // Console implementiert Output
    let console = Console {};

    // Trait-Objekte sind möglich
    let f: &dyn Output = &console;

    // println!("Output: {f}");

    // Ok, vollständig qualifizierter Pfad
    <Console as Output>::debug_tag();

    // Ok, qualifizierter Pfad
    Console::debug_tag();
}