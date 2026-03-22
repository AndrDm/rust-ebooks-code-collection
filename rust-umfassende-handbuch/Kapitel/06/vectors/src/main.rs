
fn main() {
    {
        let mut v = Vec::new();
        v.push(1);

        // Vec<T> mit T: i32
        let _ = Vec::<i32>::new();

        // Typdeklaration
        let _: Vec<i32> = Vec::new();

        // Ausreichend Speicher reservieren
        let _: Vec<i32> = Vec::with_capacity(1024);
    }

    {
        // Die From-Traits
        let slices = ["Hallo", "Rust"];
        let vektor = Vec::from(slices);
        println!("{slices:?}{vektor:?}"); // OK, Copy

        let strings = ["Hallo".to_string(), "Rust".to_string()];
        let vektor = Vec::from(strings);
        // Fehler, die Strings wurden verschoben
        // println!("{strings:?}{vektor:?}");

        let strings = ["Hallo".to_string(), "Rust".to_string()];
        let vektor = Vec::from(strings.as_slice());
        println!("{strings:?}{vektor:?}"); // OK, Clone

        let zahlen = [1, 2, 3, 4];
        let vektor = Vec::from(zahlen);
        println!("{zahlen:?}{vektor:?}"); // OK, Clone

        let eine_zeichenkette = "String zu Vektor";
        let vektor = Vec::from(eine_zeichenkette);
        // String zu Vec<u8>
        println!("{eine_zeichenkette:?}{vektor:?}"); // OK, Copy

        let zeichenkette = String::from_utf8(vektor).unwrap();
        println!("{zeichenkette}");
        // String zu Vektor
    }

    {
        // Vektoren mit anderen Collections zusammenlegen
        let mut array_1 = ['H', 'a', 'l', 'l', 'o', ' '];
        let mut array_2 = ['R', 'u', 's', 't'];

        let mut v = Vec::new();
        v.extend_from_slice(array_1.as_slice());
        // &[char; 4] -> &[char]
        v.extend_from_slice(&array_2);

        println!("{v:?}");
        // ['H', 'a', 'l', 'l', 'o', ' ', 'R', 'u', 's', 't']

        let mut v = vec![1, 2, 3];
        v.extend([4, 5, 6]);
        println!("Vektor mit Iter: {v:?}");
        // 1, 2, 3, 4, 5, 6

        // Ein Slice mit Range einengen
        let mut v: Vec<char> = Vec::new();
        v.extend(&mut array_1[0..1].iter());
        v.extend(&mut array_2[1..].iter());
        println!("{v:?}");
        // ['H', 'u', 's', 't']
    }

    {
        // Kasten IntoIterator
        let v = vec![1, 2, 3];
        for zahl in v {
            print!("{zahl}");
        }
        println!();

        struct Person {
            alter: i32,
            name: String,
            beruf: String,
        }

        let person = Person {
            alter: 1,
            name: "Thalea".to_string(),
            beruf: "Familienmanagerin".to_string(),
        };

        struct PersonIterator {
            person: Person,
            element_counter: i32,
        }
        ;

        impl Iterator for PersonIterator {
            type Item = String;

            fn next(&mut self) -> Option<Self::Item> {
                let result = match self.element_counter {
                    0 => Some(self.person.alter.to_string()),
                    1 => Some(self.person.name.clone()),
                    2 => Some(self.person.beruf.clone()),
                    _ => {
                        self.element_counter = 0;
                        None
                    }
                };
                self.element_counter += 1;
                result
            }
        }

        impl IntoIterator for Person {
            type Item = String;
            type IntoIter = PersonIterator;

            fn into_iter(self) -> Self::IntoIter {
                PersonIterator { person: self, element_counter: 0 }
            }
        }

        for eigenschaft in person {
            print!("{eigenschaft} ")
        }
        // 1 Thalea Familienmanagerin
        println!();
    }

    {
        // Vektor append
        let mut array_1 = ['H', 'a', 'l', 'l', 'o', ' '];
        let mut array_2 = ['R', 'u', 's', 't'];

        let mut v = Vec::from(array_1);

        let mut vec1 = array_2.to_vec();

        v.append(&mut vec1);
        println!("{v:?}");
        // ['H', 'a', 'l', 'l', 'o', ' ', 'R', 'u', 's', 't']
    }

    {
        // Splice
        let mut v = vec![1, 5, 6];
        println!("{v:?}"); // [1, 5, 6]

        v.splice(1.., [2, 3]);
        println!("{v:?}"); // [1, 2, 3]

        v.splice(3.., [4, 5, 6, 7]);
        println!("{v:?}"); // [1, 2, 3, 4, 5, 6, 7]

        v.splice(.., []);
        println!("{v:?}"); // []
    }

    {
        // Zugriff auf Elemente eines Vektors

        let v = vec![1, 2, 3, 4];
        let zahl = v[0];

        let elementbereich = &v[1..3];
        println!("{elementbereich:?}"); // [ 2, 3 ]

        {
            // Vorsicht Panics bei ungültigem Index
            let v: Vec<i32> = vec![];
            // Ungültiger Index
            // let zahl = v[0];

            // Ungültiger Elementbereich
            // let elementbereich = &v[1..3];
        }

        {
            // get prüft vorher
            let v: Vec<i32> = vec![1, 2, 3];

            let zahl = v.get(2); // Some(3)
            let ungueltig = v.get(4); // None

            let zahlen = v.get(0..3); // Some([1, 2, 3])
            let ungueltiger_bereich = v.get(0..30); // None

            println!(
                "Index: {zahl:?}, {ungueltig:?} \
                Elementbereich: {zahlen:?}, {ungueltiger_bereich:?}
                "
            );
        }

        {
            // Kasten: Den Index bestimmen
            let v = vec!["Hallo", ",", "Rust"];

            let index = v
                .iter()
                .position(|x| *x == ",");

            println!("Index von ',': {}", index.unwrap()); // 1
        }

        {
            // Slices aus Vektoren
            let v = vec![1, 2, 3];

            let von_as_slice = v.as_slice(); // [1, 2, 3]
            let von_indizierung = &v[..]; // [1, 2, 3]
            let von_get = v.get(..); // Some([1, 2, 3])

            println!("{von_as_slice:?}, {von_indizierung:?}, {von_get:?}");

            let mut v = vec![1, 2, 3];

            let slice = v.as_mut_slice();
            slice[0] = 3; // OK
            slice[1] = 2; // OK
            slice[2] = 1; // OK

            // Grenzen sind dennoch fest
            // slice[3] = 0; // Fehler

            let slice = v.get_mut(..);
            let slice = &mut v[..];
            slice[0] = 3; // OK
            slice[1] = 2; // OK
            slice[2] = 1; // OK

            println!("{slice:?}"); // 3, 2, 1

            // Boxed Slice
            let boxed = v.into_boxed_slice();
            println!("{:?}", &*boxed);

            // Fehler, v wurde bewegt!
            // println!("{v:?}");
        }
    }

    {
        // Elemente aus einem Vektor entfernen
        let mut v = vec![1, 2, 3];
        let zahl = v.pop(); // Some(3)
        let zahl = v.pop(); // Some(2)
        let zahl = v.pop(); // Some(1)
        let zahl = v.pop(); // None

        let mut v = vec![1, 2, 3];

        let entfernt = v.remove(1); // 2

        // Achtung Panic! Ungültiger Index
        // let _ = v.remove(4);

        {
            // Ein Beispiel zu swap_remove
            let mut v = vec![1, 2, 3, 4, 5, 6, 7];

            v.swap_remove(3);
            println!("{v:?}"); // [1, 2, 3, 7, 5, 6]

            v.swap_remove(3);
            println!("{v:?}"); // [1, 2, 3, 6, 5]
        }


        {
            // drain
            let mut v = vec![1, 2, 3, 4, 5, 6];

            {
                // Die Referenz auf v ist mit dem Block freizugeben
                let drain_iterator = v.drain(2..5);
                let entfernt = drain_iterator.as_slice();

                println!("{entfernt:?}"); // [3, 4, 5]
            }

            println!("{v:?}"); // [1, 2, 6]
        }

        {
            // retain
            let mut v = vec![1, 2, 3, 5, 8, 13, 21];
            v.retain(|x| (x & 1) == 0);

            println!("{v:?}"); // [2, 8]
        }

        {
            // dedup
            let mut zahlen = vec![1, 2, 2, 3, 3, 4, 5, 6, 7, 7, 8, 9, 9];
            zahlen.dedup();
            println!("{zahlen:?}"); // [1, 2, 3, 4, 5, 6, 7, 8, 9]

            let mut worte = vec!["Hallo", ", ", ", ", "Rust"];
            worte.dedup();
            println!("{}", worte.concat()); // Hallo, Rust

            let mut unsortiert = vec!["Hallo", ", ", "Rust", ", "];
            // Achtung, der Vektor ist nicht sortiert...
            unsortiert.dedup();
            // ... daher wird das Duplikat (,) nicht entfernt
            println!("{}", unsortiert.concat()); // Hallo, Rust,
        }

        {
            // dedup_by
            let mut v = vec![1, 2, 3, 4, 6, 8, 5, 7, 6, 8, 7, 9, 8, 9];
            v.dedup_by(|a, b| {
                let a = *a;
                let b = *b;
                // entferne a, wenn a und b beide gerade oder ungerade sind
                (a & 1 == 0) && (b & 1 == 0) || (a & 1 == 1) && (b & 1 == 1)
            });
            println!("{v:?}"); // [1, 2, 3, 4, 5, 6, 7, 8, 9]

            let mut v = vec![1, 2, 3, 4, 6, 8, 5, 7, 6, 8, 7, 9, 8, 9];
            v.dedup();
            // dedup konnte hier nichts ausrichten ...
            println!("{v:?}"); // [1, 2, 3, 4, 6, 8, 5, 7, 6, 8, 7, 9, 8, 9]

            let mut buchstaben = vec!['a', 'A', 'a', 'b', 'B', 'C', 'c'];
            buchstaben.dedup_by_key(
                |x| x.to_ascii_lowercase()
            );
            println!("{buchstaben:?}"); // ['a', 'b', 'C']
        }

        {
            // truncate
            let mut v = vec![1, 2, 3, 4];
            v.truncate(2);

            println!("{v:?}"); // [1, 2]
        }
    }


    {
        // Länge und Kapazität
        {
            let v: Vec<i32> = vec![];
            println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
            // Länge: 0, Kapazität: 0

            let v: Vec<i32> = vec![1, 2, 3];
            println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
            // Länge: 3, Kapazität: 3


            {
                // Veränderliche Vektoren mit unterschiedlichen Startkapazitäten
                println!("Vektor bei Start");
                let mut v: Vec<i32> = vec![1];
                println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
                // Länge: 1, Kapazität: 1

                println!("Vektor nachträglich");
                let mut v: Vec<i32> = vec![1];
                println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
                // Länge: 1, Kapazität: 1
                v.push(2);
                println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
                // Länge: 2, Kapazität: 4
            }

            let mut v: Vec<String> = vec!["Hallo".to_string()];
            v.push(",".into());
            v.push(" ".into());
            v.push("Rust".into());
            println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
            // Länge: 4, Kapazität: 4
            // wir gehen über die Kapazität
            v.push("!".into());
            println!("Länge: {}, Kapazität: {}", v.len(), v.capacity());
            // Länge: 5, Kapazität: 8
        }

        {
            // resize
            let mut v = vec![1, 2];
            v.resize(2, -1);
            println!("{v:?}"); // [1, 2]
            v.resize(4, -1);
            println!("{v:?}"); // [1, 2, -1, -1]
            v.resize(1, -1);
            println!("{v:?}"); // [1]


            {
                #[derive(Clone, Debug)]
                struct Person {
                    name: String,
                    alter: Option<i32>,
                }

                let mut personen: Vec<Person> = vec![];
                personen.resize(
                    3,
                    Person { name: "Unbekannt".to_string(), alter: None },
                );
                println!("{personen:?}");
                // [Person { name: "Unbekannt", alter: None }, Person { ... }, ...]
            }


            // resize_with
            let mut v = vec![];

            let mut counter = 0;
            v.resize_with(100, || {
                counter += 1;
                counter
            });
            println!("{v:?}");
            // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ... ,100]
        }

        {
            // Kapazität
            let mut v = vec![1, 2, 3];
            // Fehler: Panic, weil Kapazität zu groß
            // v.reserve(usize::MAX);

            let ergebnis = v.try_reserve(usize::MAX);
            println!("Konnte erweitern: {}", ergebnis.is_ok()); // false

            v.reserve(12); // OK
            println!("{}", v.capacity()); // 3 + 12 = 15

            let ergebnis = v.try_reserve(15);
            println!("Konnte erweitern: {}", ergebnis.is_ok()); // true
            println!("{}", v.capacity()); // 3 + 12 + 15 = 30
        }

        let mut v = vec![0; 100];
        println!("Kapazität: {}", v.capacity()); // 100
        v.truncate(1);
        println!("Kapazität: {}", v.capacity()); // 100

        v.shrink_to(50);
        println!("Kapazität: {}", v.capacity()); // 50

        v.shrink_to_fit();
        println!("Kapazität: {}", v.capacity()); // 1
    }

    {
        // VecDequeue
        use std::collections::VecDeque;

        let mut v = VecDeque::new();
        v.push_front(1);
        v.push_front(2);
        v.push_front(3);
        v.push_back(2);
        v.push_back(3);

        println!("{v:?}"); // [3, 2, 1, 2, 3]

        // Eine Queue
        let mut v = VecDeque::new();
        v.push_front("Erster");
        v.push_front("Zweiter");
        v.push_front("Dritter");

        while !v.is_empty() {
            let element = v.pop_back().unwrap();
            print!("{element}, ");
        } // Erster, Zweiter, Dritter,
        println!();
    }

    {
        // front und back
        use std::collections::VecDeque;

        let v = VecDeque::from(['A', 'B', 'C']);
        let erstes_element = v.front().unwrap();
        let letztes_element = v.back().unwrap();
        println!(
            "Erstes: {}, letztes: {}",
            erstes_element,
            letztes_element
        ); // Erstes: A, letztes: C
    }

    {
        // binary search
        use std::collections::VecDeque;

        let mut v = VecDeque::new();

        let mut zahlen: Vec<i32> = (1..10).collect();

        {
            // Die Alternative: Den generischen Parameter angeben.
            let mut zahlen = (1..10).collect::<VecDeque<_>>();
        }

        zahlen.reverse();
        zahlen.remove(4);
        println!("{zahlen:?}"); // [9, 8, 7, 6, 4, 3, 2, 1]

        for zahl in zahlen {
            let ergebnis = v.binary_search(&zahl);
            // Der Vektor ist leer, Suche gibt ein Fehlerobjekt zurück
            let sortierter_index = ergebnis.err().unwrap();

            v.insert(sortierter_index, zahl);
        }
        println!("{v:?}"); // [1, 2, 3, 4, 6, 7, 8, 9]

        let ergebnis = v.binary_search(&5)
            .err()
            .unwrap(); // 4
        v.insert(ergebnis, 5);

        println!("{v:?}"); // [1, 2, 3, 4, 5, 6, 7, 8, 9]
    }

    {
        // Binary Search By -- Gerade und ungerade Zahlen sortieren
        use std::collections::VecDeque;

        let mut v = VecDeque::new();

        let mut zahlen: Vec<i32> = (1..10).collect();
        zahlen.reverse();
        zahlen.remove(4);
        println!("{zahlen:?}"); // [9, 8, 7, 6, 4, 3, 2, 1]

        // ... bis hier wie im vorherigen Listing
        for zahl in zahlen {
            use std::cmp::Ordering;

            let ergebnis = v.binary_search_by(
                |x| {
                    if x & 1 == 0 {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                }
            );
            // Der Vektor ist leer, Suche gibt ein Fehlerobjekt zurück
            let sortierter_index = ergebnis.err().unwrap();

            v.insert(sortierter_index, zahl);
        }
        println!("{v:?}"); // [8, 6, 4, 2, 1, 3, 7, 9]

        {
            // Partition Point
            let index_gruppe_ungerade = v.partition_point(|x| x & 1 == 0);
            println!("Ungerade Zahlen starten an Index: {index_gruppe_ungerade}");
            // Ungerade Zahlen starten an Index: 4

            {
                // split_off
                let gruppe_ungerade = v.split_off(index_gruppe_ungerade);
                println!("{gruppe_ungerade:?}"); // [1, 3, 7, 9]
                let gruppe_gerade = v;
                println!("{gruppe_gerade:?}"); // [8, 6, 4, 2]
            }
        }
    }

    {
        // Rotationen
        use std::collections::VecDeque;

        let mut v: VecDeque<_> = (1..10).collect();
        // [1, 2, 3, 4, 5, 6, 7, 8, 9]
        v.rotate_left(2);
        println!("{v:?}");
        // [3, 4, 5, 6, 7, 8, 9, 1, 2]

        let mut v: VecDeque<_> = (1..10).collect();
        // [1, 2, 3, 4, 5, 6, 7, 8, 9]
        v.rotate_right(2);
        println!("{v:?}");
        // [8, 9, 1, 2, 3, 4, 5, 6, 7]
    }

    {
        // Swap
        use std::collections::VecDeque;

        let mut v = VecDeque::from([1, 2, 3, 4]);
        v.swap(0, 3);
        // [4, 2, 3, 1]
        v.swap(1, 2);
        // [4, 3, 2, 1]
    }

    {
        // Von VecDeque<T> zum Slice
        use std::collections::VecDeque;

        let v: Vec<_> = (1..10).collect();
        let slice: &[i32] = &*v;

        let mut v: Vec<_> = (1..10).collect();
        let slice: &mut [i32] = &mut *v;

        let v: VecDeque<_> = (1..10).collect();
        // Fehler, v muss veränderlich sein
        //let slice: &mut [i32] = v.make_contiguous();

        let mut v: VecDeque<_> = (1..10).collect();
        // OK
        let slice: &mut [i32] = v.make_contiguous();


        // Fehler, veränderliche Referenz auf "slice" noch aktiv
        // println!("{v:?}");
        // Fehler, Referenz von "slice" lebt zu lange
        println!("{slice:?}");
        {
            // Achtung, Lebenslinien überschneiden sich
            // let slice: &[i32] = v.make_contiguous();
            //
            // v.push_front(1);
            // println!("{slice:?}");
        }
    }

    {
        // LinkedList
        use std::collections::LinkedList;

        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let list = LinkedList::from([1, 2, 3]);

    }
}
