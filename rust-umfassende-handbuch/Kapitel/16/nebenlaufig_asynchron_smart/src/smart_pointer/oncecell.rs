use std::cell::OnceCell;

pub fn main() {
    let mut once: OnceCell<String> = OnceCell::new();

    let wert: Option<&String> = once.get(); // None

    let wert: &String = once.get_or_init(|| "Hallo, Rust".to_string());
    let wert_2: Option<&String> = once.get(); // Some(_)

    // Zwei geteilte Referenzen gültig
    println!("{wert}, {wert_2:?}");

    // Schon gesetzt...
    let ergebnis = once.set("Zu spät".into());
    println!("set: {ergebnis:?}"); // ... daher Err(_)

    // OnceCell auf nicht initialisiert zurücksetzen
    let wert = once.take();
    let ergebnis = once.set("Dieses Mal passt es".into());
    println!("set: {ergebnis:?}"); // Ok(_)

    let mut once: OnceCell<String> = OnceCell::new();
    // Veränderlicher Zugriff
    let wert: Option<&mut String> = once.get_mut(); // None

    // Fehler, exklusive Referenz durch get_mut aktiv
    let wert_2: Option<&String> = once.get();
    // println!("{wert:?}, {wert_2:?}");
}
