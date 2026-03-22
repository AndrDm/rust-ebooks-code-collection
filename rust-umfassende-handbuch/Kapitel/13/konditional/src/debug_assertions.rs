pub fn main() {
    #[cfg(debug_assertions)]
    {
        // Nur einkompilieren, wenn "Debug"
    }

    // Zur Laufzeit entscheiden
    let is_debug = if cfg!(debug_assertions) {
        // Im "Debug"-Modus ausführen
        true
    } else {
        // Das Verhalten, falls nicht "Debug"
        false
    };

    println!("Läuft im Debug-Modus: {is_debug}");
}