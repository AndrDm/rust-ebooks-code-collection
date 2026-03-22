use std::f64::consts::PI;

// const PI: f64 = 3.14;

fn main() {
    // const PI: f64 = 3.14;

    {
        let radius = 4.0;

        let kreisumfang = 2.0 * PI * radius;

        // f64.powf verrechnet mit der angegebenen Potenz
        let kreisflaeche = PI * radius.powf(2.0);

        println!("{radius}, {kreisumfang}, {kreisflaeche}");
    }

    {
        const RADIUS: f64 = 4.0;
        const KREISUMFANG: f64 = 2.0 * PI * RADIUS;

        // Fehler! radius.powf(2.0) ist weder eine Konstante,
        // noch eine konstante Funktion (const fn, NNN)
        //  const KREISFLAECHE: f64 = PI * RADIUS.powf(2.0);

        // …
        // Richtig: Einer Variablen zuweisen
        let kreisflaeche = PI * RADIUS.powf(2.0);
        println!("{RADIUS}, {KREISUMFANG}, {kreisflaeche}");
        // println!("{RADIUS}, {KREISUMFANG}, {kreisflaeche}");
    }
}
