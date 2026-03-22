extern crate core;

use std::collections::{BTreeMap, HashMap};

#[path = "../../../05/string/src/main.rs"]
pub mod kapitel_string;
mod hashes;

fn main() {
    {
        // Eine HashMap mit new erzeugen
        let mut zahl_gruppen = HashMap::new();
        zahl_gruppen.insert("prim", vec![1, 3, 5, 7, 11, 13]);
        zahl_gruppen.insert("ganze", vec![2, 4, 6, 8, 10]);
        zahl_gruppen.insert("ungerade", vec![1, 3, 5, 7, 9, 11]);

        // hier voll ausgeschrieben
        let mut zahl_gruppen = HashMap::<&str, Vec<i32>>::new();
        // ...


        // len, capacity und is_empty
        let laenge = zahl_gruppen.len();
        println!("{laenge}"); // 3

        let kapazitaet = zahl_gruppen.capacity();
        println!("{kapazitaet}"); // 3

        // is_empty
        let leere_map: HashMap<i32, i32> = HashMap::new();
        leere_map.is_empty(); // true

        zahl_gruppen.is_empty(); //false
    }

    {
        // new with capacity
        let mut zahl_gruppen = HashMap::<&str, Vec<i32>>::with_capacity(12);
        let laenge = zahl_gruppen.len();
        println!("{laenge}"); // 0

        let kapazitaet = zahl_gruppen.capacity();
        println!("{kapazitaet}"); // 14
    }

    {
        // from Trait
        let array_mit_schluesseln_und_werten = [
            ("prim", vec![1, 3, 5, 7, 11, 13]),
            ("ganze", vec![2, 4, 6, 8, 10]),
            ("ungerade", vec![1, 3, 5, 7, 9, 11])
        ];

        let map = HashMap::from(
            array_mit_schluesseln_und_werten
        );
        // ...

        let vektor_mit_paaren = vec![
            ("kleiner 10", 0..10),
            ("kleiner 20", 10..20),
            ("größer 100", 100..200),
        ];

        let map: HashMap<_, _> = std::collections::HashMap::from_iter(
            vektor_mit_paaren.into_iter()
        );
        // ...
    }

    {
        // insert
        let mut zahl_gruppen = HashMap::new();
        let ergebnis: Option<Vec<i32>> = zahl_gruppen.insert("prim", vec![1, 3, 5, 7, 11, 13]);

        println!("{ergebnis:?}"); // None
        let ergebnis = zahl_gruppen.insert("prim", vec![1, 3, 5]);
        // Wir erhielten den alten Eintrag zurück...
        println!("{ergebnis:?}"); // Some([1, 3, 5, 7, 11, 13])

        let primzahlen = &zahl_gruppen["prim"]; // [1, 3, 5]
        println!("Primzahlen in Map: {primzahlen:?}");


        // Fehler, HashMap implementiert IndexMut nicht
        // zahl_gruppen["prim"] = vec![1, 3, 5, 7, 11, 13];
        // zahl_gruppen["ganze"] = vec![2, 4, 6, 8, 10];
        // zahl_gruppen["ungerade"] = vec![1, 3, 5, 7, 9, 11];
    }

    {
        // reserve, try_reserve
        let mut zahl_gruppen: HashMap<i32, i32> = HashMap::new();
        zahl_gruppen.reserve(12);

        let kapazitaet = zahl_gruppen.capacity(); // 14
        println!("Kapazität nach reserve: {kapazitaet}");
    }

    {
        // remove
        let mut map = HashMap::from(
            [
                (1, "Hallo"),
                (2, "Rust"),
            ]
        );

        let ergebnis = map.remove(&3); // None
        println!("{ergebnis:?}");

        let ergebnis = map.remove(&1); // Some("Hallo")
        println!("{ergebnis:?}");

        // remove_entry
        let mut map = HashMap::from(
            [
                (1, "Hallo"),
                (2, "Rust"),
            ]
        );

        let entry = map.remove_entry(&1);
        let key = entry.unwrap().0; // 1
        let value = entry.unwrap().1; // Hallo
        println!("Entfernt: {key} : {value}");

        let entry = map.remove_entry(&2);
        // Als Tupel-Muster auslesen
        let (key, value) = entry.unwrap();
        println!("Entfernt: {key} : {value}"); // 2 : Rust

        // clear
        let mut map = HashMap::from(
            [
                (1, "Hallo"),
                (2, "Rust"),
            ]
        );

        println!("{}", map.len()); // 2
        map.clear();
        println!("{}", map.len()); // 0

        // drain
        let mut map = HashMap::from(
            [(1, "Hallo"), (2, "Rust")]
        );

        {
            // Lebenszeit von drain-Referenz beschränken
            let entries = map.drain();

            // [(2, "Rust"), (1, "Hallo")]
            println!("{entries:?}");
        }


        let referenz_auf_map = &map;
        println!("Leer nach drain: {}", referenz_auf_map.is_empty());
        // true

        // retain
        let mut map = HashMap::from([
            (1, "Hallo"), (2, "Rust"), (3, "Halte mich")]
        );
        map.retain(|k, v| v.starts_with("H"));
        println!("{map:?}");
        // {1: "Hallo", 3: "Halte mich"}
    }

    {
        // Auf Einträge zugreifen

        // contains_key
        let map = HashMap::from(
            [(1, "Hallo"), (2, "Rust")]
        );
        let ergebnis = map.contains_key(&2); // true
        if ergebnis {
            let wert = map[&2];
            // ...
        }

        let ergebnis = map.contains_key(&22); // false
        // ...


        // get und get_mut
        {
            let erster_eintrag = map.get(&1);
            println!("{erster_eintrag:?}"); // Some("Hallo")
            let unbekannt = map.get(&11);
            println!("{unbekannt:?}"); // None
        }

        {
            let mut map = HashMap::from(
                [(1, "Hallo"), (2, "Rust")]
            );
            let erster_eintrag = map.get_mut(&1);
            let wert = erster_eintrag.unwrap();
            println!("{}", *wert); // Hallo
            *wert = "Hola";
            println!("{}", *wert); // Hola
        }

        // get_key_value
        {
            let map = HashMap::from(
                [(1, "Hallo"), (2, "Rust")]
            );
            let (key, value) = map.get_key_value(&2).unwrap();
            println!("Schlüssel: {}, Wert: {}", key, *value);
            // Schlüssel: 2, Wert: Rust
        }
    }

    {
        // Iteratoren
        let mut map = HashMap::from(
            [(1, "Hallo"), (2, "Rust")]
        );

        for key in map.keys() {
            // ...
        }

        // map.values() analog dazu
        for value in map.values_mut() {
            // ...
        }

        // map.iter() analog dazu
        for entry in map.iter_mut() {
            println!("Schlüssel: {}, Wert: {}", entry.0, entry.1);
        }

        // Achtung, hier wird die Map jeweils konsumiert, daher clone()!
        let keys = map.clone().into_keys();
        // ...
        let values = map.clone().into_values();
        // ...
        let iterator = map.into_iter();
        // ...
    }

    {
        use kapitel_string::Bestellung;

        // Erweiterung aus Kapitel 6, "Collections und Slices"
        impl Bestellung {
            fn from_string_with_map(
                bestelltext: &str,
                speisen: &HashMap<i32, &str>,
            ) -> Bestellung {
                // Wir nutzen den Parser aus Kapitel 5
                let bestellung: Bestellung = bestelltext.parse().unwrap();

                // Prüfen, ob die Sorte als Zahl oder Name eingebracht wurde
                let sorte = bestellung.sorte;
                let speise_nummer: i32 = sorte.parse().unwrap_or_default();

                // get liefert eine Referenz auf &str, daher &&str
                let speise_name: Option<&&str> = speisen.get(&speise_nummer);

                Bestellung {
                    anzahl: bestellung.anzahl,
                    // Haben wir eine Zahl?
                    // Dann Map-Wert, ansonsten einfach die Eingabe nehmen
                    sorte: speise_name.unwrap_or(&sorte.as_str()).to_string(),
                    original: bestellung.original,
                }
            }
        }

        let bestellung_salami: Bestellung = "1 x 77".parse().unwrap();
        println!("{bestellung_salami:?}"); // OK
        // Bestellung { anzahl: 1, sorte: "Pizza Salami", original: "1 x 77" }

        // Problem, eine feste Verdrahtung von 77 und Salami besteht
        let bestellung_tonno: Bestellung = "1 x 78".parse().unwrap();
        println!("{bestellung_tonno:?}"); // Fehler
        // Bestellung { anzahl: 1, sorte: "78", original: "1 x 78" }

        // Veränderlich, falls nachträglich etwas geändert werden soll
        let mut speisen = HashMap::from([
            // ... vorherige Speisen
            (77, "Pizza Salami"),
            (78, "Pizza Tonno"),
            (103, "Gartensalat"),
            (104, "Salat Mista"),
            (105, "Beilagensalat"),
            (200, "Pizzabrot"),
            // ... nachfolgende Speisen
        ]);

        let bestellung_salami = Bestellung::from_string_with_map(
            "1 x 77",
            &speisen,
        );
        // Bestellung { anzahl: 1, sorte: "Pizza Salami", original: "1 x 77" }
        println!("{bestellung_salami:?}");

        let bestellung_tonno = Bestellung::from_string_with_map(
            "1 x 78",
            &speisen,
        );
        // Bestellung { anzahl: 1, sorte: "Pizza Tonno", original: "1 x 78" }
        println!("{bestellung_tonno:?}");

        let unbekannte_bestellung = Bestellung::from_string_with_map(
            "1 x 9999",
            &speisen,
        );
        // Bestellung { anzahl: 1, sorte: "9999", original: "1 x 9999" }
        println!("{unbekannte_bestellung:?}");

        speisen.retain(|k, _| (100..=199).contains(k) == false);
        // {78: "Pizza Tonno", 77: "Pizza Salami", 200: "Pizzabrot"}
        println!("{speisen:?}");
    }

    {
        // BTreeMap
        let zahlen = (1..=10).collect::<Vec<_>>();
        let zahlen_tuples = zahlen.iter().map(
            |x| (x, x * 10)
        ).collect::<Vec<_>>();
        let mut b_baum = BTreeMap::from_iter(zahlen_tuples);
        println!("{b_baum:?}");
        // {1: 10, 2: 20, 3: 30, 4: 40, 5: 50, 6: 60, 7: 70, 8: 80, 9: 90, 10: 100}

        // Auch möglich: range_mut
        let ausschnitt = b_baum.range(2..5);
        // [(2, 20), (3, 30), (4, 40)]
        println!("{ausschnitt:?}");

        let other = &mut BTreeMap::from(
            [(&100, 1000), (&200, 2000)]
        );
        b_baum.append(
            other
        );
        println!("{b_baum:?}");
        // {..., 9: 90, 10: 100, 100: 1000, 200: 2000}

        {
            // Feature first und last
            {
                // first_key_value und last_key_value
                let b_baum = BTreeMap::from(
                    [(1, "A"), (2, "B"), (3, "C")]
                );
                let eintrag = b_baum
                    .first_key_value()
                    // Wir erhalten ein Option<(_,_)>
                    .unwrap(); // (1, "A")
                println!("first_key_value: {eintrag:?}");

                let eintrag = b_baum
                    .last_key_value()
                    // Wir erhalten ein Option<(_,_)>
                    .unwrap(); // (3, "C")
                println!("last_key_value: {eintrag:?}");
            }

            {
                // first_entry und last_entry

                // Mit „mut“ einführen, da first_entry und last_entry
                // eine veränderliche Referenz auf BTreeMap fordern
                let mut b_baum = BTreeMap::from(
                    [(1, "A"), (2, "B"), (3, "C")]
                );

                let eintrag = b_baum
                    .first_entry()
                    // Wir erhalten ein Option<OccupiedEntry<_>>
                    .unwrap(); // Eintrag: 1, "A"
                println!("Schlüssel: {}, Wert: {}", eintrag.key(), eintrag.get());

                let eintrag = b_baum
                    .last_entry()
                    // Wir erhalten ein Option<OccupiedEntry<_>>
                    .unwrap(); // Eintrag: 3, "C"
                println!("Schlüssel: {}, Wert: {}", eintrag.key(), eintrag.get());
            }

            {
                // pop_first und pop_last

                // Auch hier „mut“, da die pop-Operationen
                // die BTreeMap verändern
                let mut b_baum = BTreeMap::from(
                    [(1, "A"), (2, "B"), (3, "C")]
                );

                let erster_entfernt = b_baum.pop_first();
                let letzter_entfernt = b_baum.pop_last();

                println!("{b_baum:?}"); // {2: "B"}
            }

        }
    }

    {
        // Die Entry API

        {
            {
                // Ohne Entry API
                let mut zustaende = HashMap::from(
                    [("Fenster offen", true), ("Türe offen", true), ]
                );

                if zustaende.contains_key("Licht an") {
                    let licht = zustaende["Licht an"];
                    // ...
                } else {
                    zustaende.insert("Licht an", false);
                }

                let sensor = zustaende.get("Annäherungssensor aktiv");
                if sensor.is_some() {
                    let licht = sensor.unwrap();
                } else {
                    zustaende.insert("Annäherungssensor aktiv", false);
                }
                // ...
            }

            let mut zustaende = HashMap::from(
                [("Fenster offen", true), ("Türe offen", true), ]
            );

            let tuere = zustaende.entry("Türe offen");
            // Ist bereits in der Map
            println!("Türe offen: {}", tuere.or_insert(false));
            // Türe offen: true

            let licht = zustaende.entry("Licht an");
            // Fehlt in der Map
            let licht_sensor = licht.or_insert(false);
            println!("Licht an: {}", licht_sensor);
            // Licht an: false

            // Mit der veränderlichen Referenz überschreiben
            *licht_sensor = true;
            println!("Licht an: {}", licht_sensor);
            // Licht an: true

            {
                // Die Referenzen und Lebenszeiten
                let licht = zustaende.entry("Licht an");
                // Leiht eine veränderliche Referenz auf den Wert aus
                let wert = licht.or_insert(false);

                // Eine Referenz auf die Map ausleihen
                // println!("{zustaende:?}");

                // Achtung, Fehler! Nun lebt "wert" (mut) weiter.
                println!("{wert:?}");
            }
        }

        {
            // Occupied und Vacant
            let mut zahl_gruppen = HashMap::new();

            let eintrag_vacant = zahl_gruppen.entry("gerade");
            println!("{eintrag_vacant:?}"); // Entry(VacantEntry("gerade"))

            // Einen Eintrag mit Entry einfügen
            eintrag_vacant.or_insert(vec![2, 4, 6]);
            println!("{zahl_gruppen:?}");
            // {"gerade": [2, 4, 6]}

            let eintrag_occupied = zahl_gruppen.entry("gerade");
            println!("{eintrag_occupied:?}");
            // Entry(OccupiedEntry { key: "gerade", value: [2, 4, 6], .. })

            {
                // VacantEntry und OccupiedEntry auflösen
                use std::collections::hash_map::{Entry, VacantEntry};

                let mut zahl_gruppen: HashMap<&str, Vec<i32>> = HashMap::new();
                let eintrag = zahl_gruppen.entry("gerade");

                match eintrag {
                    // Musterabgleich: Könnte eintrag in die Variante
                    // Vacant mit Bezeichner "freier_eintrag" eingefügt werden?
                    Entry::Vacant(freier_eintrag) => {
                        // jetzt weiß der Compiler, dass es sich um VacantEntry handelt
                        freier_eintrag.insert(vec![2, 4, 6]);
                    }
                    // Musterabgleich: Könnte eintrag in die Variante
                    // Occupied mit Bezeichner "besetzter_eintrag" eingefügt werden?
                    Entry::Occupied(besetzter_eintrag) => {
                        // jetzt weiß der Compiler, dass es sich um OccupiedEntry handelt
                        besetzter_eintrag.remove();
                    }
                }
            }
        }


        {
            // Weitere inserts
            let mut zahl_gruppen = HashMap::new();
            let eintrag = zahl_gruppen.entry("gerade");

            // Eine anonyme Funktion berechnet den Wert
            let wert: &mut Vec<i32> = eintrag.or_insert_with(|| vec![2, 4, 6]);

            // ...

            let mut zahl_gruppen = HashMap::new();
            let eintrag = zahl_gruppen.entry("gerade");
            // Eine anonyme Funktion berechnet den Wert und
            // bekommt den Schlüssel als Parameter
            let wert: &mut Vec<usize> = eintrag.or_insert_with_key(|key| {
                let mut zahlen = vec![2, 4, 6];
                // Vom Vektor einen veränderlichen Iterator anfordern
                // mehr dazu in Kapitel 1.8
                zahlen.iter_mut().for_each(|x| *x = *x * key.len());
                zahlen
            });
            // {"gerade": [12, 24, 36]}
            println!("{zahl_gruppen:?}");

            // ...

            // Hier müssen die Datentypen von Schlüssel und Wert
            // angegeben werden. Sonst kann default unten nicht
            // aufgelöst werden.
            let mut zahl_gruppen: HashMap<&str, i32> = HashMap::new();
            let eintrag = zahl_gruppen.entry("gerade");
            // Den Standardwert des Datentyps einfügen
            let wert: &mut i32 = eintrag.or_default();
        }

        {
            // and_modify
            // Es muss sich ein Wert für den Schlüssel in der Map befinden
            let mut zahl_gruppen = HashMap::from([("gerade", vec![2, 4, 6])]);
            // {"gerade": [2, 4, 6]}
            println!("{zahl_gruppen:?}");

            let eintrag = zahl_gruppen.entry("gerade");
            let aktualisierter_eintrag = eintrag.and_modify(
                // Vom Vektor einen veränderlichen Iterator anfordern
                // mehr dazu in Kapitel 1.8
                |vec: &mut Vec<i32>| vec.iter_mut().for_each(|n| *n = *n * 10)
            );
            // {"gerade": [20, 40, 60]}
            println!("{zahl_gruppen:?}");
        }

        {
            // and_modify und or_insert in Reihe
            let mut zahl_gruppen = BTreeMap::new();
            let eintrag = zahl_gruppen.entry("gerade");

            eintrag
                .and_modify(|vec: &mut Vec<i32>| vec.append(&mut vec![8, 10, 12]))
                .or_insert(vec![2, 4, 6]);
            // {"gerade": [2, 4, 6]}
            println!("{zahl_gruppen:?}");
        }

        {
            // Mehrere Datentypen in eine Collection mit Enum
            {
                enum Sammlung {
                    Ganzzahl(i32),
                }

                let zahl = Sammlung::Ganzzahl(21);

                let anzahl_bytes = std::mem::size_of_val(&zahl); // 4
                println!("Bytes in Sammlung::Gannzahl: {anzahl_bytes}");
            }

            {
                enum Sammlung {
                    Ganzzahl(i32),
                    Fliesskommzahl(f32)
                }

                let zahl = Sammlung::Ganzzahl(21);

                let anzahl_bytes = std::mem::size_of_val(&zahl); // 8
                println!("Bytes in Sammlung::Gannzahl: {anzahl_bytes}");
            }

            // Mehrere Strukturen für die Enum einführen
            #[derive(Eq, Hash, PartialEq)]
            struct Pizza { radius: i32 }
            #[derive(Eq, Hash, PartialEq)]
            struct Salat { vegetarisch: bool}
            #[derive(Eq, Hash, PartialEq)]
            struct Steak { herkunft: String }

            #[derive(Eq, Hash, PartialEq)]
            enum Speise {
                Pizza(Pizza),
                Salat(Salat),
                Steak(Steak),
                Pizzabrot,
            }

            let mut speisen = Vec::<Speise>::new();
            speisen.push(Speise::Pizza(Pizza { radius: 36}));
            speisen.push(Speise::Salat(Salat { vegetarisch: true} ));
            speisen.push(Speise::Salat(Salat { vegetarisch: false} ));
            speisen.push(Speise::Steak(Steak { herkunft: "Argentinien".into() }));
            speisen.push(Speise::Pizzabrot);

            let anzahl_speisen = speisen.len();
            println!("Bekannte Speisen: {anzahl_speisen}");

            let mut bestellungen = HashMap::<Speise, i32>::new();
            let pizza_24 = bestellungen.entry(Speise::Pizza(Pizza { radius: 24 }));
            let bestellte_pizzas_24 = pizza_24
                .and_modify(|zaehler| *zaehler += 1)
                .or_insert(0);

            let steak = bestellungen.entry(
                Speise::Steak(Steak { herkunft: "Deutschland".into()})
            );
            let bestellte_steaks = steak
                .and_modify(|zaehler| *zaehler += 1)
                .or_insert(0);

            let pizzabrot = bestellungen.entry(Speise::Pizzabrot);
            let bestellte_pizzabrote = pizzabrot
                .and_modify(|zaehler| *zaehler += 1)
                .or_insert(0);
        }
    }

    {
        // HashSets
        use std::collections::HashSet;

        {
            // HashSet ist eine HashMap mit () als Value
            let mut hash_menge = HashSet::new();
            hash_menge.insert(1);
            hash_menge.insert(2);

            // Keine Duplikate möglich
            hash_menge.insert(2);
            println!("{hash_menge:?}"); // {1, 2}

            let mut hash_map = HashMap::new();
            // Als Wert übergeben wir den Unit Typ
            hash_map.insert(1, ());
            hash_map.insert(2, ());

            // Keine Duplikate möglich
            hash_map.insert(2, ());
            println!("{hash_menge:?}"); // {1, 2}

        }

        {
            // Mengenoperationen
            let untere_menge = (1..=5).collect::<HashSet<_>>();

            let obere_menge = (5..=10).collect::<HashSet<_>>();


            let unterschied = untere_menge.difference(&obere_menge);
            println!("{unterschied:?}"); // [3, 2, 1, 4]

            let schnittmenge = untere_menge.intersection(&obere_menge);
            println!("{schnittmenge:?}"); // [5]

            {
                // is_disjoint
                let nichts_gemeinsam = untere_menge.is_disjoint(&obere_menge);
                println!("{nichts_gemeinsam}"); // false, die 5 existiert in beiden

                {
                    let untere_menge = (1..5).collect::<HashSet<_>>();
                    let nichts_gemeinsam = untere_menge.is_disjoint(&obere_menge);
                    println!("{nichts_gemeinsam}");  // true
                }
            }

            {
                // is_subset und is_superset
                let untermenge = (1..5).collect::<HashSet<_>>();
                let obermenge = (1..10).collect::<HashSet<_>>();
                let ist_untermenge = untermenge.is_subset(&obermenge); // true
                println!("Ist Untermenge: {ist_untermenge:?}");

                let ist_obermenge = obermenge.is_superset(&untermenge); // true
                println!("Ist Obermenge: {ist_obermenge:?}");
            }


            {
                // Mengen zusammenführen
                let erste_menge = (1..=8).collect::<HashSet<_>>();
                let zweite_menge = (3..=10).collect::<HashSet<_>>();

                {
                    // symmetric_difference
                    let entweder_oder = erste_menge.symmetric_difference(&zweite_menge);
                    println!("{entweder_oder:?}"); // [2, 1, 9, 10]
                }

                {
                    // union
                    let vereinigung = untere_menge.union(&obere_menge);
                    println!("{vereinigung:?}"); // [9, 5, 6, 7, 10, 8, 4, 3, 2, 1]

                    use std::collections::BTreeSet;
                    let neue_menge = vereinigung.collect::<BTreeSet<_>>();
                    // {1, 2, 3, 4, 5, 6, 7, 8, 9, 10}
                    println!("{neue_menge:?}");
                }
            }
        }
    }

    hashes::main();
}
