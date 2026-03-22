pub fn main() {
    // Beispiele für chars
    let buchstabe_r = 'r';
    let violine = '\u{1D11E}'; // 𝄞

    // Beispiel: Warnung, ungültig
    // let b = '\u{D8FF}';

    // Beispiel für unterschiedliche Bytegrößen
    use std::mem::size_of_val;

    // Beispiel für gleiche Bitbreite
    let buchstabe_r = 'r';
    let violine = '\u{1D11E}'; // 𝄞
    println!(
        "Char {} besitzt Bytelänge: {}",
        buchstabe_r,
        size_of_val(&buchstabe_r)
    );
    println!(
        "Char {} besitzt Bytelänge: {}",
        violine,
        size_of_val(&violine)
    );

    let violine_im_string = String::from("\u{1D11E}");
    let buchstabe_r_im_string = String::from("r");
    // Das Zeichen 𝄞 benötigt 4 Byte im String
    println!(
        "String {} besitzt Bytelänge: {}",
        violine_im_string,
        size_of_val(&violine_im_string[..])
    );
    // Das Zeichen r aber nur 1 Byte im String
    println!(
        "String {} besitzt Bytelänge: {}",
        buchstabe_r_im_string,
        size_of_val(&buchstabe_r_im_string[..])
    );

    // Beispiel: Konvertieren
    let violine = '\u{1D11E}';
    let a = violine as i8;
    let b = violine as i16;
    let c = violine as i128;
    let d = violine as u8;
    let e = violine as u16;
    let f = violine as u128;

    println!(
        "Buchstabe: {}, \
         i8: {}, \
         i16: {}, \
         i128: {}, \
         u8: {}, \
         u16: {}, \
         u128: {}",
        violine, a, b, c, d, e, f
    );

    let c = 127 as char;
    let x = 0xFF as char;

    println!("{}, {}", c, x);

    // Beispiele von Integern und Strings zu char
    let c = char::from_u32(65);
    // wir holen mit unwrap den Wert aus Option
    // dazu später mehr
    println!("from u_32: {}", c.unwrap());

    let c = char::from(65);
    // hier kein Option, u8 kann nicht scheitern
    println!("from u_8: {}", c);

    // from_str in den Sichtbarkeitsbereich holen
    use std::str::FromStr;

    let c = char::from_str("A");
    // wir holen mit unwrap den Wert aus Option
    // dazu später mehr
    println!("from String: {}", c.unwrap());
}
