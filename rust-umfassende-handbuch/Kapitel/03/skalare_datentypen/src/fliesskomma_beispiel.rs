pub fn main() {
    // Beispiel: f64 zeigt mehr Nachkommastellen
    use std::f64::consts::PI;

    // Wir nehmen PI aus f64 und
    // pressen es in ein f32 hinein
    let single: f32 = PI as f32;
    let double: f64 = PI;

    println!("Single: {}\nDouble: {}\n", single, double);

    let ohne_exponent = 9.256; // e^0
    let mit_exponent = 9.256e3;
    let mit_negativem_exponent = 9.256e-1;

    println!(
        "Ohne: {}, mit: {}, mit Negativem: {}",
        ohne_exponent, mit_exponent, mit_negativem_exponent
    );

    {
        // Typsuffix
        let a = 0.2_f32; // f32
        let b = 0.2_f64; // f64
        let c = 0.2; // Standard: f64
    }

    {
        // Ungenauigkeiten mit 0,1

        // 0.1_f32 + 0.2_f32 => 0.3
        println!("{}", 0.3_f32 == 0.1_f32 + 0.2_f32); // true

        // 0.1_f64 + 0.2_f64 => 0.30000000000000004
        println!("{}", 0.3 == 0.1_f64 + 0.2_f64); // false !

        // Kein Problem bei anderen Werten!
        println!("{}", 2.6 == 0.1_f64 + 2.5_f64); // true

        // Epsilon f32: 0.00000011920929
        println!("Epsilon f32: {}", f32::EPSILON);
        // Epsilon f64: 0.0000000000000002220446049250313
        println!("Epsilon f64: {}", f64::EPSILON);

        let double_precision = 0.1_f64 + 0.2_f64;
        let difference = double_precision - 0.3;
        println!("{}", difference <= f64::EPSILON); // true
    }
}
