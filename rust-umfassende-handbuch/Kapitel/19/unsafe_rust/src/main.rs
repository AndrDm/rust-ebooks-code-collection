#![allow(unused_variables)]
#![allow(unused_unsafe)]

use std::alloc::Layout;

fn main() {
    let v = vec![1, 2];

    // Ok, aber Option<T> möchten wir vermeiden
    let zahl: Option<&i32> = v.get(1);

    // Ok, unsafe-Signal behandelt
    unsafe {
        let zahl: &i32 = v.get_unchecked(1);
    }

    {
        // Unsafe-Traits

        // Angenommen, "Person" implementiert
        // "Send" und "Sync" nicht automatisch
        struct Person;

        unsafe impl Send for Person {
            // Die Probleme behandeln
        }

        unsafe impl Sync for Person {
            // Die Probleme behandeln
        }
    }

    static mut GLOBALER_ZAEHLER: i32 = 0;

    // Unsicher, daher unsafe-Block
    unsafe {
        println!("Zähler: {GLOBALER_ZAEHLER}");
        GLOBALER_ZAEHLER = 1;
        println!("Zähler: {GLOBALER_ZAEHLER}");
    }

    {
        // Hauptweg führt über Referenz
        let str = "Hallo, Rust";

        let p_str: *const str = str as *const str;
        println!("Adresse: {p_str:p}");
        // Adresse: 0x104b0feec
    }

    {
        // Von Box zu zeiger
        // "into_raw" belässt den Wert im Heap, überträgt das Eigentum
        // und die Pficht zur Freigabe jedoch auf den Zeiger
        let p_str: *mut &str = Box::into_raw(Box::new("Hallo, Rust"));

        // Mit p_str arbeiten ...

        // Fertig, also drop
        unsafe {
            std::ptr::drop_in_place(p_str);
        }
    }

    {
        // as_ptr bei Smart Pointern

        let str = "Hallo, Rust";
        // "as_ptr" von "str"
        let p_str: *const u8 = str.as_ptr();

        use std::cell::Cell;
        use std::rc::Rc;

        let str_cell = Cell::new("Hallo, Rust");
        // Dereferenzierung mit vollqualifiziertem
        // Pfad auf "Cell" vermeiden
        let p_str_cell: *const &str = Cell::as_ptr(&str_cell);

        let str_rc = Rc::new("Hallo, Rust");
        // Dereferenzierung mit vollqualifiziertem
        // Pfad auf "Rc" vermeiden
        let p_str_rc: *const &str = Rc::as_ptr(&str_rc);
    }

    {
        // Makros

        let mut str = "Hallo, Rust";

        let p_str: *const &str = std::ptr::addr_of!(str);
        let p_mut_str: *mut &str = std::ptr::addr_of_mut!(str);
    }

    {
        // Null-Zeiger

        // Einen konstanten Zeiger erstellen
        let p: *const i32 = std::ptr::null();
        unsafe {
            println!("Adresse Null: {p:p}");
            // Adresse Null: 0x0
        }

        let p: *mut i32 = std::ptr::null_mut();
        unsafe {
            println!("Adresse Null-Mut: {p:p}");
            // Adresse Null-Mut: 0x0
        }

        let zahl = 42;
        let p_zahl = &zahl as *const i32;

        assert_eq!(p_zahl.is_null(), false);

        let p: *mut i32 = std::ptr::null_mut();
        assert_eq!(p.is_null(), true);
    }

    {
        // Zeiger-Operationen
        let mut zahl = 42;
        let mut zahl_2 = 42;
        let p_zahl = &mut zahl as *mut i32;

        unsafe {
            // Im Speicher vorangehen
            let p_zahl_2 = p_zahl.offset(1);
            *p_zahl_2 = 21;
        }

        let p_zahl_2 = &mut zahl_2 as *mut i32;
        unsafe {
            // Im Speicher zurückgehen
            let p_zahl_2 = p_zahl_2.wrapping_offset(-1);
            *p_zahl_2 = 7;
        }

        println!("Zahl: {zahl}");
        println!("Zahl 2: {zahl_2}");

        {
            // Version mit add und sub

            let mut zahl = 42;
            let mut zahl_2 = 42;
            let p_zahl = &mut zahl as *mut i32;

            unsafe {
                // Im Speicher vorangehen
                let p_zahl_2 = p_zahl.add(1);
                *p_zahl_2 = 21;
            }

            let p_zahl_2 = &mut zahl_2 as *mut i32;
            unsafe {
                // Im Speicher zurückgehen
                let p_zahl_2 = p_zahl_2.sub(1);
                *p_zahl_2 = 7;
            }

            println!("Zahl: {zahl}");
            println!("Zahl 2: {zahl_2}");
        }
    }

    {
        // read

        let zahl = 42;
        let p_zahl = &zahl as *const i32;
        {
            let kopie = unsafe { p_zahl.read() };

            println!("Original: {zahl}, Kopie: {kopie}");
        } // <- Gibt "kopie" frei

        // "zahl" weiterhin gültig

        let str = std::mem::ManuallyDrop::new(String::from("Hallo, Rust"));
        unsafe {
            let str_kopie = std::ptr::read(&*str);

            println!("Original: {}, Kopie: {str_kopie}", *str);
        } // <- Gibt durch "str_kopie" den Speicher von "str" im Heap frei!

        // <- Will "str" freigeben, aber String schon freigegeben: Double Free!

        // Programm bricht ab:
        // unsafe_rust(89952,0x1e3eed000) malloc: *** error for object 0x600001a7c060: pointer being freed was not allocated
        // unsafe_rust(89952,0x1e3eed000) malloc: *** set a breakpoint in malloc_error_break to debug
    }

    {
        // write

        // Verweist auf den originalen String-Puffer im Heap
        let mut p_str_original: *mut str;
        {
            let mut str = String::from("Hallo, Rust");

            // String -> Puffer im Heap -> &str
            p_str_original = &mut *str as *mut str;

            let neuer_str = String::from("Neuer String!");
            unsafe {
                let p_str = &mut str as *mut String;
                std::ptr::write(p_str, neuer_str);

                println!("Original: {}, Überschrieben: {}", &*p_str_original, &*str);
                // Original: Hallo, Rust,
                // Überschrieben: Neuer String!
            };
        } // <- drop von "str"

        // Original weiterhin im Speicher!
        unsafe {
            // Original manuell freigeben
            // Den Drop-Destruktor aufrufen
            std::ptr::drop_in_place(p_str_original);
            // Das Speicherobjekt im Heap freigeben
            std::alloc::dealloc(
                p_str_original as *mut u8,
                Layout::for_value(&*p_str_original),
            );

            // Jetzt undefiniertes Verhalten: Ausgabe nicht klar
            println!("Original: {}", &*p_str_original);
            // Original: Hallo, Rust
        }
    }

    {
        // Fallstrick Move

        {
            // Variante Stack
            struct Puffer {
                v: [u8; 4],
                leser: *const u8,
            }

            impl Puffer {
                pub fn new() -> Puffer {
                    let v = [1, 2, 3, 4];
                    let leser = v.as_ptr();

                    let mut p = Puffer { v, leser };

                    // Wert im Puffer ändern
                    p.v[0] = 4;

                    // Puffer zurückgeben
                    p
                }
            }

            let p = Puffer::new();
            unsafe {
                // assert_eq!(*p.leser, 4); // Fehler!
                assert_eq!(p.v[0], 4); // Ok
            }
        }

        {
            // Variante Heap
            struct Puffer {
                v: Vec<u8>,
                leser: *const u8,
            }

            impl Puffer {
                pub fn new() -> Puffer {
                    let v = vec![1, 2, 3, 4];
                    let leser = v.as_ptr();

                    let mut p = Puffer { v, leser };

                    // Wert im Puffer ändern
                    p.v[0] = 4;

                    // Puffer zurückgeben
                    p
                }
            }

            let p = Puffer::new();
            unsafe {
                assert_eq!(*p.leser, 4); // Fehler!
                assert_eq!(p.v[0], 4); // Ok
            }
        }
    }

    {
        // Union
        union Zeiger {
            ist_gesetzt: bool,
            adresse: *const u8,
        }

        let z_leer = Zeiger {
            adresse: std::ptr::null(),
        };
        unsafe {
            matches!(z_leer.ist_gesetzt, false);
            println!("Adresse: {:p}", z_leer.adresse);
        }

        let z_leer = Zeiger { ist_gesetzt: false };
        unsafe {
            matches!(z_leer.ist_gesetzt, false);
            println!("Adresse: {:p}", z_leer.adresse);
        }

        let z_gesetzt = Zeiger {
            adresse: &42 as *const i32 as *const u8,
        };
        unsafe {
            matches!(z_gesetzt.ist_gesetzt, true);
            println!("Adresse: {:p}", z_gesetzt.adresse);

            match z_gesetzt {
                Zeiger { ist_gesetzt: true } => {
                    // ...
                }
                Zeiger { adresse } => {
                    // ...
                }
            }

            if let _ = z_gesetzt.ist_gesetzt {
                // ...
            }
        }

        {
            // Referenzen

            {
                struct ZeigerStruct {
                    ist_gesetzt: bool,
                    adresse: *const u8,
                }

                let mut zeiger_struct = ZeigerStruct {
                    ist_gesetzt: false,
                    adresse: std::ptr::null(),
                };

                // Struct verwaltet Referenzen getrennt für jedes Feld
                // &zeiger_struct.ist_gesetzt kann gleichzeitig mit
                // &mut zeiger_struct.adresse gültig sein
                let ref_ist_gesetzt_struct = &zeiger_struct.ist_gesetzt;
                // Beugung in *const u8 findet automatisch statt
                zeiger_struct.adresse = &42;
                println!("Gesetzt in Struct: {ref_ist_gesetzt_struct:?}");
            }

            {
                // Das trifft jedoch nicht auf Unions zu!
                let mut zeiger_union = Zeiger {
                    adresse: &42 as *const i32 as *const u8,
                };

                unsafe {
                    let ref_ist_gesetzt_union = &zeiger_union.ist_gesetzt;

                    // Fehler, Union ist durch Referenz oben gesperrt!
                    // zeiger_union.adresse = &42;
                    // println!("Gesetzt in Struct: {ref_ist_gesetzt_union:?}");
                }
            }
        }
    }
}
