use std::collections::HashMap;

fn main() {
    let tuple = (1, 'A', "Rust");

    let zahl = tuple.0;
    let zeichen = tuple.1;
    let string = tuple.2;
    println!("{zahl}, {zeichen}, {string}");
    // 1, 'A', "Rust"

    {
        let (zahl, zeichen, string) = tuple;
        println!("{zahl}, {zeichen}, {string}");
        // 1, A, Rust
    }
    {
        // ... veränderlich
        let (mut zahl, zeichen, string) = tuple;
        println!("{zahl}, {zeichen}, {string}");
        // 1, A, Rust
        zahl = 2;
        println!("{zahl}, {zeichen}, {string}");
        // 2, A, Rust

        {
            // Copy und Move
            let tuple = (1, 'A', "Rust".to_string());
            let (mut zahl, zeichen, string) = tuple;
            // Fehler, der String wurde schon herausbewegt
            // let rust_string = tuple.2;
        }

        {
            // Schlüsselwort ref führt zur (veränderlichen) Referenz
            let mut tuple = (1, 'A', "Rust".to_string());
            let (ref mut zahl, zeichen, string) = tuple;

            // veränderlich durch ref mut
            *zahl = 2;

            // "zeichen" ist eine Kopie, "string" herausbewegt
            println!("Tuple: {},{},{}", zahl, zeichen, string);

            // OK, tuple.0 wurde aufgrund der Referenz
            // (ref zahl) nicht herausbewegt
            println!("Zahl in Tupel geändert auf: {}", tuple.0);
        }

        {
            // mut Tupel -> der Datentyp muss übereinstimmen
            let mut tuple = (1, 'A', "Rust");

            // OK
            tuple = (2, 'B', "Hallo");

            // Fehler, unterschiedliche Tupel-Typen
            // tuple = ("Falsche", "Typen", 2.34);
        }
    }

    {
        // Der Unterstrich als Platzhalter
        {
            let (zahl, _, string) = tuple;
            println!("{zahl}, {string}");
            // 1, Rust
        }
        {
            let (zahl, zeichen, _) = tuple;
            println!("{zahl}, {zeichen}");
            // 1, A
        }
        {
            // Alle ignorieren
            let (_, _, _) = tuple;

            println!("{tuple:?}");
            // 1, A
        }
    }

    {
        let zahlen = (1, 2, 3, 4, 5, 6, 7, 8, 9);
        let (erste, .., letzte) = zahlen;
        println!("{erste}, {letzte}");
        // 1, 9
    }

    let gleiches_tupel = (1, 'A', "Rust");

    println!(
        "Die Tuples sind gleich: {}",
        tuple == gleiches_tupel
    ); // true

    let weiteres_tupel = (2, 'B', "Hallo");

    println!(
        "Die Tupel sind gleich: {}",
        tuple == weiteres_tupel
    ); // false

    {
        // let zahlen = (1.0, 2.0, 3.0, 5.0);
        // // Fehler, Tupel müssen gleichen Typ haben
        // println!(
        //     "Die Tupel sind gleich: {}",
        //     // kompiliert nicht!
        //     tuple == zahlen
        // );
    }

    {
        // Elementwerte in einem Tuple verändern
        let mut tuple = (0, "");

        // OK, neue Werte und gleiche Datentypen
        tuple.0 = 1;
        tuple.1 = "Hallo, Rust";
        println!("{:?}", tuple);
        // (1, "Hallo, Rust")

        // Fehler, abweichende Datentypen
        // tuple.0 = "Ups";
        // tuple.1 = 42;
    }

    {
        // Tuples können in andere Collections eingefügt werden
        let mut vec = vec![weiteres_tupel, tuple, gleiches_tupel];
        println!("{vec:?}");
        // [(2, 'B', "Hallo"), (1, 'A', "Rust"), (1, 'A', "Rust")]

        {
            // Tuples gleichen Typs können auch miteinander verglichen werden
            println!("tuple < gleiches_tuple: {}", tuple < gleiches_tupel);
            // tuple < gleiches_tuple: false
            println!("tuple < gleiches_tuple: {}", tuple < weiteres_tupel);
            // tuple < gleiches_tuple: true


            println!("{vec:?}");
            // [(2, 'B', "Hallo"), (1, 'A', "Rust"), (1, 'A', "Rust")]

            vec.sort();
            println!("{vec:?}");
            // [(1, 'A', "Rust"), (1, 'A', "Rust"), (2, 'B', "Hallo")]
        }
    }

    {
        // Mit einem Tuple lässt sich schnell eine IP Adresse erfassen
        use std::net::ToSocketAddrs;

        let tuple = ("192.168.0.100", 80);
        let socket_addr = tuple
            .to_socket_addrs()
            // Result
            .unwrap()
            .last()
            // Option
            .unwrap();
        println!("{socket_addr}");
        // 192.168.0.100:80
    }

    {
        // Der Unit-Typ

        let a = 2;
        let b = 2;

        let kein_wert: () = if a + b == 4 {
            println!("Der Wert ist 4");
        } else {
            println!("Da ist etwas schiefgelaufen!");
        };

        let block: () = {
            println!("{}", a + b);
        };

        fn eine_funktion() -> () {
            println!("Hier gibt es nichts zurückzugeben");
        }
    }
}
