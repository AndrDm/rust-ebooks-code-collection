pub fn main() {
    {
        // Beispiele für Speicherausdrücke
        let mut zahlen = [1, 2, 3];
        // Der Indizierungs-Ausdruck produziert
        // einen Speicherbereich
        zahlen[1] = 5;

        let mut tupel = (1, 'A');
        // ... ebenso der Tupel-Ausdruck
        tupel.0 = 2;
        tupel.1 = 'B';

        struct Person {
            alter: i32,
        }
        let mut p = Person { alter: 41 };

        // Ein weiteres Beispiel: der Feld-Ausdruck
        p.alter = p.alter + 1;
    }

    {
        // Ein Speicherausdruck wertet je nach Kontext aus
        let mut x = 1;

        // Wertet zu einer Adresse aus
        x = 2;

        // ebenfalls eine Adresse
        x += 2;

        // Als Unterausdruck wertet x zu seinem Wert aus
        let y = x + 2;

        // Nochmal x als Unterausdruck
        if x + 2 < 10 {
            println!("Kleiner 10!");
        } else {
            println!("Größer 10");
        }
    }

    {
        // Ein Wertausdruck verbraucht den Speicherausdruck

        let zahlen = [1, 2, 3];
        for zahl in zahlen {
            print!("{zahl}");
        }
        println!();

        // OK, Zahlen ist Copy
        println!("{zahlen:?}");

        // ...

        let zahlen = ["1", "2", "3"]
            .map(String::from);

        for zahl in zahlen {
            print!("{zahl}");
        }
        println!();

        // Fehler, zahlen wurde verbaucht
        // println!("{zahlen:?}")
    }

    {
        // Eine temporäre Variable

        let str = "Hallo, Rust!";
        // let zeichen = str.to_string().bytes();

        // Fehler, der String aus to_string ist schon freigegeben
        // println!("{zeichen:?}");
    }
}