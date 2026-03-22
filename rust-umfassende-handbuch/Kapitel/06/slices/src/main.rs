fn main() {
    {
        // Ein Slice in Stücke aufteilen
        let zahlen_vektor = (1..12).collect::<Vec<_>>();
        let slice = zahlen_vektor.as_slice();

        {
            // Chunks
            use std::slice::Chunks;

            let chunks: Chunks<i32> = slice.chunks(3);
            let chunks_exact = slice.chunks_exact(3);

            println!("Chunks");
            for chunk in chunks {
                print!(" {chunk:?} ");
            }
            println!();

            println!("Chunks exact");
            for chunk in chunks_exact.clone() {
                print!(" {chunk:?} ");
            }
            println!();
            println!("Übrig: {:?}", chunks_exact.remainder());

            let chunks = slice.chunks(30);
            println!("Chunkgröße 30 ergibt {} Teile", chunks.count());

            let chunks = slice.chunks_exact(30);
            println!("Chunkgröße 30 ergibt {} Teile", chunks.clone().count());
            println!("Anzahl Elemente in remainder {}", chunks.remainder().len());
        }

        {
            // Windows
            let windows = slice.windows(3);
            // [1, 2, 3] [2, 3, 4] [3, 4, 5] [4, 5, 6]
            // [5, 6, 7] [6, 7, 8] [7, 8, 9] [8, 9, 10] [9, 10, 11]
            println!("Windows");
            for window in windows {
                print!("{window:?}");
            }
            println!();

            let windows = slice.windows(30); // leer
            println!("Chunkgröße 30 ergibt {} Teile", windows.count());
        }

        {
            // Elemente zusammenführen

            // concat

            // Ok, concat für &str
            println!("{}", ["Hallo", ", ", "Rust"].concat());
            // Hallo, Rust

            // OK, concat auf einem zweidimensionalen Array
            println!("{:?}", [[1, 2], [3, 4]].concat());
            // [1, 2, 3, 4]

            // Fehler, die Arrays müssen den gleichen Typ ([i32; 2]) haben
            // println!("{:?}", [[1, 2], [3], [4]].concat());

            // Fehler, keine Slices in Slices
            // println!("{}", [1, 2, 3, 4].concat());

            // join

            println!("{}", ["Hallo", "Rust"].join(", "));
            // Hallo, Rust

            println!("{:?}", [['a'], ['b'], ['c']].join(&'d'));
            // ['a', 'd', 'b', 'd', 'c']


            let string: String = ["Hallo", ", ", "Rust"].concat();
            let vektor: Vec<i32> = [[1, 2], [3, 4]].concat();

            let string: String = ["Hallo", ", ", "Rust"].join("-");
            let vektor: Vec<i32> = [[1, 2], [3, 4]].join(&9);

            use std::borrow::Borrow;

            struct Person {
                name: String,
                lieblingszahlen: Vec<u32>,
            }

            impl Borrow<str> for Person {
                fn borrow(&self) -> &str {
                    self.name.as_str()
                }
            }

            impl Borrow<[u32]> for Person {
                fn borrow(&self) -> &[u32] {
                    self.lieblingszahlen.as_slice()
                }
            }

            let person_1 =
                Person { name: "Thalea".into(), lieblingszahlen: vec![1, 2, 3] };
            let person_2
                = Person { name: "Anne".into(), lieblingszahlen: vec![5, 7] };

            let familie_namen = [person_1, person_2].join::<&str>(", ");
            println!("{familie_namen:?}"); // "Thalea, Anne"

            // ...

            let person_1 = Person { name: "Thalea".into(), lieblingszahlen: vec![1, 2, 3] };
            let person_2 = Person { name: "Anne".into(), lieblingszahlen: vec![5, 7] };
            let lieblingszahlen = [person_1, person_2].concat::<u32>();
            println!("{lieblingszahlen:?}"); // [1, 2, 3, 5, 7]


            {
                // borrow allgemein
                let person_1 = Person { name: "Thalea".into(), lieblingszahlen: vec![1] };

                let a: &str = person_1.borrow();
                let b: &[u32] = person_1.borrow();
            }
        }

        {
            // repeat
            println!("{:?}", [1, 2, 3].repeat(4));
            // [1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3]
        }

        {
            // split
            let zahlen_vektor = (1..12).collect::<Vec<_>>();

            let slice = zahlen_vektor.as_slice();
            let slices = slice.split(|x| x % 5 == 0);
            for slice in slices {
                println!("{slice:?}");
            }
            // Trenner ist nicht in Ergebnis!
            // [1, 2, 3, 4]
            // [6, 7, 8, 9]
            // [11]

            let splits = slice.splitn(3, |x| x % 2 == 0);
            for split in splits {
                println!("{:?}", split);
            }
            // [1]
            // [3]
            // [5, 6, 7, 8, 9, 10, 11]

            let splits = slice.split_inclusive(|x| x % 5 == 0);
            for split in splits {
                println!("{:?}", split);
            }
            // [1, 2, 3, 4, 5]
            // [6, 7, 8, 9, 10]
            // [11]

            // split_at
            let slices = slice.split_at(zahlen_vektor.len() / 2);
            println!("Erste Hälfte: {:?}, zweite Hälfte: {:?}", slices.0, slices.1);
            // Erste Hälfte: [1, 2, 3, 4, 5], zweite Hälfte: [6, 7, 8, 9, 10, 11]
        }
    }

    {
        // len und is_empty
        let zahlen = &*(1..4).collect::<Vec<_>>();
        println!("Länge: {}", zahlen.len());

        let leeres_array: [i32; 0] = [];
        // Array nutzt automatisch is_empty von Slice
        println!(
            "Ist leer: {}, Länge: {}",
            leeres_array.is_empty(),
            leeres_array.len()
        );
    }

    {
        // contains
        println!("{}", ["Nadel", "im", "Heuhaufen"].contains(&"Nadel"));
        // true

        println!("{}", [1, 2, 3, 5, 8].contains(&7));
        // false

        // starts_with und ends_with
        println!("Startet mit 3, 2, 1?: {}", [1, 2, 3].starts_with(&[3, 2, 1]));
        // false
        println!("Startet mit 1, 2?: {}", [1, 2, 3].starts_with(&[1, 2]));
        // true

        println!("Endet mit 3, 2, 1?: {}", [1, 2, 3].ends_with(&[3, 2, 1]));
        // false
        println!("Endet mit 1, 2?: {}", [1, 2, 3].ends_with(&[2, 3]));
        // true
    }

    {
        // Elemente referenzieren

        {
            // Indizierungs-Ausdruck
            let vec = (1..11).collect::<Vec<_>>();
            let slice = vec.as_slice();

            // i32 ist Copy
            let i = slice[2];
            // Panic! Ungültiger Index
            // let j = slice[50];
            // ...

            let array = ["Hallo".to_string(), "Rust".to_string()];
            let slice = array.as_slice();
            // Referenz verhindert Move, der nicht möglich wäre
            let s = &slice[1];
            // ...
        }

        {
            // first, last und get
            let vec = (1..11).collect::<Vec<_>>();
            let slice = vec.as_slice();

            let optional_erster = slice.first();
            // Some(1)
            let optional_letzter = slice.last();
            // Some(10)

            println!(
                "Erster: {}, letzter: {}",
                optional_erster.unwrap(),
                optional_letzter.unwrap()
            );

            let i = slice.get(2);
            // Some(3)
            let j = slice.get(50);
            // None

            println!("i: {i:?}, j: {j:?}");

            let mut vec = (1..11).collect::<Vec<_>>();
            let slice = vec.as_mut_slice();
            let erste = slice.first_mut();
            *(erste.unwrap()) = 99;
            // [99, 2, 3, 4, 5, 6, 7, 8, 9, 10]
            println!("{slice:?}");
        }
    }

    {
        // Veränderungen im Slice durchführen

        {
            // copy_from_slice
            let mut vec = (1..5).collect::<Vec<_>>();
            let slice = vec.as_mut_slice();

            // OK
            slice.copy_from_slice(&[9, 9, 9, 9]);
            // [9, 9, 9, 9]
            println!("{slice:?}");

            // Fehler, die Längen müssen übereinstimmen
            // slice.copy_from_slice(&[]);
            // Fehler, die Längen müssen übereinstimmen
            // slice.copy_from_slice(&[1, 2, 3, 4, 5]);
        }

        {
            // copy_within
            let mut vec = ('A'..'I').collect::<Vec<_>>();
            let slice = vec.as_mut_slice();
            // ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']
            println!("{slice:?}");

            // OK
            slice.copy_within(0..3, 3);
            // ['A', 'B', 'C', 'A', 'B', 'C', 'G', 'H']

            // Achtung, Panic! Bereich größer als die Länge
            // slice.copy_within(0..30, 3);

            // Panic, Zielpunkt außerhalb des Slices
            // slice.copy_within(0..3, slice.len());

            // Panic, Kopiervorgang geht über Slicegrenze
            // slice.copy_within(0..3, slice.len() - 2);


            println!("{slice:?}");
        }

        {
            // fill
            let mut array = [0, 0, 0, 0];
            let slice = array.as_mut_slice();

            slice.fill(8);
            // [8, 8, 8, 8]
            println!("{slice:?}");
        }

        {
            // fill_with
            let mut array = [0, 0, 0, 0];
            let slice = array.as_mut_slice();

            {
                // Ein Block begrenzt die Sichtbarkeit von count
                let mut count = 0;
                slice.fill_with(|| {
                    let value = count * 3;
                    count += 1;
                    value
                });
                // [0, 3, 6, 9]
            }

            println!("{slice:?}");
        }
    }

    {
        // reverse, sort, swap
        let mut array = [1, 2, 3, 4];
        let slice = array.as_mut_slice();
        slice.reverse(); // [4, 3, 2, 1]
        println!("Reverse: {slice:?}");

        // durch reverse [4, 3, 2, 1]
        slice.sort();
        // nach sort: [1, 2, 3, 4]
        println!("Sort: {array:?}");

        #[derive(Ord, PartialOrd, Eq, PartialEq)]
        struct EinEigenerTyp { i: i32}

        let mut array = [
            EinEigenerTyp {i: 2},
            EinEigenerTyp {i: 1}
        ];
        let slice = array.as_mut_slice();
        slice.sort();

        {
            let mut array = [1, 2, 3, 4, 5, 6, 7];
            let slice = array.as_mut_slice();
            // Sortiere gerade Zahlen nach links, ungerade nach rechts
            slice.sort_by(|x, y| {
                use std::cmp::Ordering;

                if x % 2 == 0 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            // [6, 4, 2, 1, 3, 5, 7]
            println!("Sort By: {array:?}");
        }

        {
            let mut array = [1, 2, 3, 4, 5, 6, 7];
            let slice = array.as_mut_slice();
            // Sortiere die Elemente nach Rest 0, 1, 2
            slice.sort_by_key(|x| x % 3);
            // [3, 6, 1, 4, 7, 2, 5]
            println!("Sort by key: {array:?}");
        }


        let mut array = [1, 2, 3, 4];
        let slice = array.as_mut_slice();

        // Tausche Index 1 mit 2
        slice.swap(1, 2); // [1, 3, 2, 4]
        println!("{slice:?}");

        // Achtung, ungültige Indizes führen zur Panic
        // slice.swap(10, 200);
    }

    {
        // ASCII

        let mut vec = vec![65, 66, 67, 68];
        let slice = vec.as_mut_slice();
        // ABCD

        slice.make_ascii_lowercase();
        // abcd
        println!("{}", String::from_utf8(slice.to_vec()).unwrap());

        let mut array = [97, 98, 99, 100];
        let array_slice = array.as_mut_slice();
        //abcd

        {
            // Vergleiche
            let vec_grossbuchstaben = vec![65, 66, 67, 68];
            let grossbuchstaben = vec_grossbuchstaben.as_slice();

            let array_kleinbuchstaben = [97, 98, 99, 100];
            let kleinbuchstaben = array_kleinbuchstaben.as_slice();

            println!(
                "Sind gleich: {}",
                grossbuchstaben.eq_ignore_ascii_case(kleinbuchstaben)
            ); // true
            println!(
                "Sind gleich: {}",
                grossbuchstaben == kleinbuchstaben
            ); // false
        }

        array_slice.make_ascii_uppercase();
        //ABCD
        println!("{}", String::from_utf8(array_slice.to_vec()).unwrap());

        {
            // Prüfung, ob ASCII
            let array = [97, 98, 99, 100];
            let array_slice = array.as_slice();
            println!("Ist ASCII: {}", array_slice.is_ascii()); // true

            let array = [200, 97, 98, 99, 100, 253, 254];
            let array_slice = array.as_slice();

            println!("Ist ASCII: {}", array_slice.is_ascii()); // false
        }

        {
            // Escapen
            let slice = b"\t\tDoppelt Eingerueckt\nNeue Zeile";

            println!(
                "Unmaskiert: {}",
                String::from_utf8(slice.to_vec()).unwrap()
            );
            // Unmaskiert: 		Doppelt Eingerueckt
            // Neue Zeile

            println!(
                "Maskiert: {}",
                slice.escape_ascii().to_string()
            );
            // Maskiert: \t\tDoppelt Eingerueckt\nNeue Zeile
        }
    }
}