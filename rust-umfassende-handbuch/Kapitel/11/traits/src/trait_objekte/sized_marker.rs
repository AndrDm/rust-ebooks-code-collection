
trait Form {}

struct Punkt {
    x: i32,
    y: i32,
}

impl Form for Punkt {}

pub fn main() {
    // Referenz auf den breiten Zeiger "dyn Form"
    let form: &dyn Form = &Punkt { x: 1, y: 2 };

    // Box<T> unterstützt Werte ohne feste Größe
    let form: Box<dyn Form> = Box::new(Punkt { x: 1, y: 2 });

    {
        // Fehlerfälle
        fn verarbeiten<T>(t: &T) {}

        // Ok, i32
        verarbeiten(&42);

        // Ok, String
        verarbeiten(&"Hallo, Rust".to_string());

        // Ok, &Punkt
        verarbeiten(&Punkt { x: 1, y: 2 });

        // Fehler, T ist ein DST (&dyn Form)
        // verarbeiten(&*form);

        // Fehler, T ist ein DST (&str)
        // verarbeiten("");

        // Fehler, T ist ein DST (&[i32])
        // verarbeiten(&*[1, 2, 3].as_slice());
    }

    {
        // Kompatibel mit ?Sized

        fn verarbeiten<T: ?Sized>(t: &T) {
            // ...
        }

        // ...

        // Ok
        verarbeiten(&*form);

        // Ok
        verarbeiten("");

        // Ok
        verarbeiten(&*[1, 2, 3].as_slice());
    }


}