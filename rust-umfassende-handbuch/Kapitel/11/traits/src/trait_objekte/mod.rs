
// trait Form {}

mod sized_marker;
mod einschraenkungen;
mod traits_auf_trait_objekten;
mod inhaerent_trait_objekt;

// vTable Assembler-Code Beispiel
trait Form {
    fn methode_a(&self) {}
    fn methode_b(&self) {}
    fn methode_c(&self) {}
}

struct Punkt {
    x: i32,
    y: i32,
}

struct Linie {
    a: Punkt,
    b: Punkt,
}

struct Rechteck {
    a: Linie,
    b: Linie,
}

struct Kreis {
    p: Punkt,
    r: f64,
}

// Implementierungen
impl Form for Punkt {}
impl Form for Linie {}
impl Form for Rechteck {}
impl Form for Kreis {}

pub fn main() {

    {
        // Funktion mit Polymorphie über Form
        // Fehler, Traits haben keine feste Größe
        // fn zeichne(form: Form) {
        //
        // }

        fn zeichne(form: &dyn Form) {

        }

        zeichne(&Punkt { x: 1, y: 2 });
    }

    {
        // Konvertierung zurück nicht möglich
        // Ok, von Instanz zu Trait-Objekt beugen
        let form: &dyn Form = &Punkt { x: 1, y: 2 };
        // Fehler
        // let punkt = form as Punkt;

        // Ok, von Instanz zu Trait-Objekt beugen
        (&Punkt { x: 1, y: 2 } as &dyn Form).methode_a();
    }

    traits_auf_trait_objekten::main();
    inhaerent_trait_objekt::main();

    // Trait-Objekte besitzen keine feste Größe (DST)
    // Entweder hinter einer Referenz oder einem Smart Pointer
    let formen: Vec<&dyn Form> = vec![];

    let slice = formen.as_slice();
    let a: [&dyn Form; 1] = [&Punkt { x: 1, y: 1 }];

    // Ein Trait-Objekt entsteht, indem der Compiler eine Referenz oder einen Wert
    // in einen breiten Zeiger übersetzt.
    {
        let punkt: Box<dyn Form> = Box::new(Punkt { x: 1, y: 1 });
        // Referenz auf temporäre Stackvariable
        let punkt_referenz: &dyn Form = &Punkt { x: 1, y: 1 };
    }


}