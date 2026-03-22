#![allow(unused_variables)]



fn main() {
    {
        // Iterator beziehen
        let v = vec![1, 2, 3, 4, 5];
        let iterator: std::slice::Iter<i32> = v.iter();

        // e: &i32
        for e in iterator {
            // ...
        }

        let mut v = vec![1, 2, 3, 4, 5];
        let iterator_mut: std::slice::IterMut<i32> = v.iter_mut();

        // e: &mut i32
        for e in iterator_mut {
            // ...
        }

        let v = vec![1, 2, 3, 4, 5];
        let w: std::vec::IntoIter<i32> = v.into_iter();

        let mut a = [1, 2, 3, 4, 5];
        let iterator: std::slice::Iter<i32> = a.iter();
        let iterator_mut: std::slice::IterMut<i32> = a.iter_mut();
        // ...

        {
            // HashMap
            use std::collections::{hash_map, HashMap};

            let h = HashMap::from([(1, "Hallo")]);
            let iterator: hash_map::Iter<i32, &str> = h.iter();

            let mut h = HashMap::from([(1, "Rust")]);
            let iterator: hash_map::IterMut<i32, &str> = h.iter_mut();
        }
    }

    {
        // IntoIterator und for
        let v = vec![1, 2, 3, 4, 5];
        for e in v {
            // ...verbraucht "v" und die Elemente (Move)
        }
        // Fehler, "v" wurde zuvor bewegt
        // println!("Vektor: {v:?}");

        let v = vec![1, 2, 3, 4, 5];
        let v_ref = &v;

        let iterator = v.iter();
        for e in iterator {
            // ...
        }

        for e in v_ref {
            // ...
        }

        // Ok
        println!("Vektor: {v:?}");
        // Fehler, "iterator" wurde
        // durch Move in for verbraucht
        // println!("Vektor: {iterator:?}");
    }

    {
        // Iterator auf eigenem Datentyp

        // Struktur Person
        struct Person {
            alter: i32,
            name: String,
            beruf: String,
        }

        impl Person {
            fn iter(&self) -> PersonIterator {
                PersonIterator::new(&self)
            }

            fn iter_mut(&mut self) -> PersonIteratorMut {
                PersonIteratorMut::new(self)
            }
        }

        // Iter
        struct PersonIterator<'a> {
            // Der Iterator soll die Instanz
            // nicht verbrauchen können.
            person: &'a Person,
            // "Cell", um Interior Mutability auszunutzen
            // Dazu mehr in Abschnitt 18.2.2
            _printed_elements: std::cell::Cell<i32>,
        }

        impl PersonIterator<'_> {
            pub fn new(p: &Person) -> PersonIterator {
                PersonIterator {
                    person: p,
                    _printed_elements: Default::default(),
                }
            }
        }

        impl Iterator for PersonIterator<'_> {
            type Item = String;
            fn next(&mut self) -> Option<Self::Item> {
                let result = match self._printed_elements.get() {
                    0 => Some(self.person.alter.to_string()),
                    1 => Some(self.person.name.clone()),
                    2 => Some(self.person.beruf.clone()),
                    _ => {
                        self._printed_elements.set(0);
                        None
                    }
                };
                self._printed_elements.set(self._printed_elements.get() + 1);
                result
            }
        }

        impl<'a> IntoIterator for &'a Person {
            type Item = String;
            type IntoIter = PersonIterator<'a>;

            fn into_iter(self) -> Self::IntoIter {
                PersonIterator::new(self)
            }
        }

        // IterMut

        struct PersonIteratorMut<'a> {
            // Der Iterator soll die Instanz
            // nicht verbrauchen können.
            person: &'a mut Person,
            // "Cell", um Interior Mutability auszunutzen
            // Dazu mehr in Abschnitt 18.2.2
            printed_elements: std::cell::Cell<i32>,
        }

        impl PersonIteratorMut<'_> {
            pub fn new(p: &mut Person) -> PersonIteratorMut {
                PersonIteratorMut {
                    person: p,
                    printed_elements: Default::default(),
                }
            }
        }

        impl Iterator for PersonIteratorMut<'_> {
            type Item = String;
            fn next(&mut self) -> Option<Self::Item> {
                let result = match self.printed_elements.get() {
                    0 => Some(self.person.alter.to_string()),
                    1 => Some(self.person.name.clone()),
                    2 => Some(self.person.beruf.clone()),
                    _ => {
                        self.printed_elements.set(0);
                        None
                    }
                };
                self.printed_elements.set(self.printed_elements.get() + 1);
                result
            }
        }

        impl<'a> IntoIterator for &'a mut Person {
            type Item = String;
            type IntoIter = PersonIteratorMut<'a>;
            fn into_iter(self) -> Self::IntoIter {
                PersonIteratorMut::new(self)
            }
        }

        // Verwendung
        let mut person = Person {
            alter: 3,
            name: "Thalea".to_string(),
            beruf: "Familienmanagerin".to_string(),
        };

        for eigenschaft in person.iter() {
            print!("{eigenschaft} ");
        }
        println!();

        for eigenschaft in &person {
            print!("{eigenschaft} ");
        }
        println!();

        for eigenschaft in person.iter_mut() {
            print!("{eigenschaft} ");
        }
        println!();

        {
            // Kasten: FromIterator
            use std::collections::HashMap;
            let h: HashMap<i32, &str> = HashMap::from_iter([(1, "Hallo"), (2, "Rust")]);
        }
    }

    {
        // Iterator Adapter

        let v = vec![1, 2, 3, 4, 5, 6, 7];
        let positive_zahlen = v
            // Iterator aus Vektor erstellen
            .into_iter()
            // Zwei am Anfang überspringen
            .skip(2)
            // Dann nimm vier Zahlen
            .take(4)
            // Zwischenstand überprüfen
            .inspect(|x| println!("Elemente: {x}"))
            // Verarbeite nur die Positiven weiter
            // Dereferenzierung, da n: &i32
            .filter(|n| *n % 2 == 0)
            // Kehre die Reihenfolge umlet positive_zahlen = v
            .rev()
            // Führe den Iterator aus
            .collect::<Vec<_>>();

        println!("{positive_zahlen:?}");
        // [6, 4]

        // Wenn collect fehlt:
        // Rev { iter: Filter {
        //         iter: Inspect {
        //             iter: Take {
        //                 iter: Skip {
        //                     iter: IntoIter(
        //                         [1, 2, 3, 4, 5, 6, 7]
        //                     ),
        //                     n: 2
        //                 },
        //                 n: 4
        //             }
        //         }
        //     }
        // }

        {
            // skip_while und take_while
            let zahlen = vec![1, 2, 3, 4, 5, 6, 7]
                .into_iter()
                .skip_while(|n| *n < 3)
                .take_while(|n| *n < 6)
                .collect::<Vec<_>>();

            // [3, 4, 5]
            println!("{zahlen:?}");
        }
    }

    {
        // Eigentum im Iterator

        {
            // cloned und copied
            let zahlen = vec![1, 2, 3, 4, 5, 6, 7];
            let iterator = zahlen
                .iter()
                // Jedes Element kopieren
                .copied()
                // Transformiert Typ "T" in "U"
                // oder verändert den Wert im Iterator
                .map(|n| n * n)
                .collect::<Vec<i32>>();
            println!("Zahlen: {zahlen:?}");
            println!("Iterator: {iterator:?}");

            let worte = ["Hallo", ",", "Rust", "!"]
                .iter()
                // &str zu String transformieren
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let iterator = worte
                .iter()
                // Jedes Element klonen
                .cloned()
                // Der Iterator ist jetzt Eigentümer der Klone
                .map(|s| s.to_ascii_uppercase())
                .collect::<Vec<_>>();

            let zip_iterator = worte.iter().cloned().zip(iterator).collect::<Vec<_>>();

            println!("Wort und Klon: {zip_iterator:?}");
            // Wort und Klon: [("Hallo", "HALLO"), (",", ","), ("Rust", "RUST"), ("!", "!")]

            let (originale, klone): (Vec<_>, Vec<_>) = zip_iterator.iter().cloned().unzip();
            // Originale:
            //  ["Hallo", ",", "Rust", "!"],
            // Klone:
            //  ["HALLO", ",", "RUST", "!"]
            println!("Originale: {originale:?}, Klone: {klone:?}")
        }

        {
            // by_ref

            let satzteile = ["Hallo", ",", "Rust", "!"]
                .iter()
                // &str zu String transformieren
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let mut iterator = satzteile.into_iter();

            let worte = iterator
                .by_ref()
                .filter(|s| s.len() > 1)
                .collect::<Vec<_>>();

            let worte_gross = iterator
                .by_ref()
                .map(|s| s.to_ascii_uppercase())
                .collect::<Vec<_>>();

            // ... mit "iterator" weiterarbeiten
        }
    }

    {
        // flatten und chain
        let erster_teil = [1, 2, 3, 4, 5]
            .into_iter();
        let zweiter_teil = [6, 7, 8, 9, 10]
            .into_iter();

        let alle_zahlen = [erster_teil, zweiter_teil]
            .into_iter();
        let alle_flach = alle_zahlen
            .flatten()
            .collect::<Vec<i32>>();
        println!("Flach: {alle_flach:?}");
        // Flach: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

        // ...

        let erster_teil = [1, 2, 3, 4, 5]
            .into_iter();
        let zweiter_teil = [6, 7, 8, 9, 10]
            .into_iter();
        let gekettet = erster_teil
            .chain(zweiter_teil)
            .collect::<Vec<_>>();
        println!("Kette: {gekettet:?}");
        // Kette: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    }

    {
        // enumerate
        ["Hallo", ",", "Rust", "!"]
            .iter()
            // Die nächsten Adapter erhalten (Index, Element)
            .enumerate()
            .for_each(
                |paar| {
                    println!(
                        "Index: {}, Element: {}",
                        paar.0,
                        paar.1
                    );
                }
            );
        // Index: 0, Element: Hallo
        // Index: 1, Element: ,
        // Index: 2, Element: Rust
        // Index: 3, Element: !
    }

    {
        // Mehr zu maps
        let zahlen = [1, 2, 3, 4, 5]
            .into_iter()
            .map_while(|n| {
                if n <= 3 {
                    Some(n.to_string())
                } else {
                    None
                }
            }).collect::<Vec<_>>();
        // map_while: ["1", "2", "3"]
        println!("map_while: {zahlen:?}");

        let zahlen = [1, 2, 3, 4, 5]
            .into_iter()
            // Die Zahlen 2 bis 4 in Strings umwandeln
            .filter_map(|n| {
                if n >= 2 && n <= 4 {
                    Some(n.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        // filter_map: ["2", "3", "4"]
        println!("filter_map: {zahlen:?}");

        let erster_teil = ["Hallo", ","]
            .into_iter();
        let zweiter_teil = [" Rust", "!"]
            .into_iter();
        let alle_zahlen = [erster_teil, zweiter_teil]
            .into_iter()
            // Nimm IntoIterator entgegen, gib abgeflacht zurück
            .flat_map(|n| n.enumerate())
            .collect::<Vec<_>>();
        // flat_map: [(0, "Hallo"), (1, ","), (0, " Rust"), (1, "!")]
        println!("flat_map: {alle_zahlen:?}");
    }

    {
        // Iterator konsumieren
        let zahlen = [1, 2, 3, 4, 5]
            .into_iter();

        // Anzahl der Elemente
        assert_eq!(zahlen.clone().count(), 5);
        // Erfüllt mindestens eins der Elemente die Bedingung?
        assert_eq!(zahlen.clone().any(|n| n == 3), true);
        // Erfüllen alle Elemente die Bedingung?
        assert_eq!(zahlen.clone().all(|n: i32| n % 2 == 0), false);
        // Gib das letzte Element des Iterators zurück
        assert_eq!(zahlen.clone().last(), Some(5));
        // Der kleinste Wert im Iterator
        assert_eq!(zahlen.clone().min(), Some(1));
        // Der größte Wert im Iterator
        assert_eq!(zahlen.clone().max(), Some(5));
        // Führe die Closure für jedes Element aus
        zahlen.clone().for_each(|n| println!("Element: {n}"));

        // Rufe "FromIterator" für collect::<T> auf
        let kopie = zahlen.clone().collect::<Vec<_>>();

        // Lese das n-te Element aus
        assert_eq!(zahlen.clone().nth(3), Some(4));
    }

    {
        // find, fold und reduce
        let mut zahlen = [1, 2, 3, 4, 5]
            .into_iter();

        // Findet und entnimmt das Element. Ansonsten "None"
        assert_eq!(zahlen.find(|n| n == &4 || n % 2 == 1 ), Some(1));
        // [2, 3, 4, 5]
        println!("Zahlen: {zahlen:?}");

        let mut zahlen = ["1", "2", "3"].into_iter();
        assert_eq!(zahlen.find_map(|n| {
               use std::str::FromStr;

               i32::from_str(n).ok()
           }),
           Some(1)
        );

        {
            // fold
            let zahlen = [1, 2, 3, 4, 5]
                .into_iter();

            assert_eq!(
                zahlen.fold(10, |mut summe, n| {
                        summe += n;
                        summe
                    }),
                25 // 10 + 2 + 3 + 4 + 5
            );
            let v = vec!["Hallo, ", "Rust!"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();
            assert_eq!(
                v.iter().fold(String::new(), |text, s| {
                    use std::ops::Add;
                    text.add(&s)
                }),
                "Hallo, Rust!".to_string()
            );
        }

        {
            // reduce
            let zahlen = [1, 2, 3, 4, 5]
                .into_iter();

            assert_eq!(
                zahlen
                    .reduce(|mut summe, n| {
                        summe += n;
                        summe
                    }),
                Some(15) // 2 + 3 + 4 + 5
            );

            let v = vec!["Hallo, ", "Rust!"]
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            assert_eq!(
                v.into_iter().reduce(|text, s| {
                    use std::ops::Add;
                    text.add(&s)
                }),
                Some("Hallo, Rust!".to_string())
            );
        }


    }
}
