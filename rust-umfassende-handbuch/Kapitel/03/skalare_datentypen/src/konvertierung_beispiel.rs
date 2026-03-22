use std::str::FromStr;

pub fn main() {
    // Das höchstwertige Byte hat nur 7 Bit,
    // das 8. ist für das Vorzeichen.
    let number = 0b111_1111_1111_1111_1111_1111_0000_0001;
    let smaller = number as i8;

    println!("i32: {number}, i8: {smaller}");
    // Ausgabe: i32: 2147483392, i8: 0

    // u32 in i32 konvertieren
    let unsigned_number = 0b11111111_11111111_11111111_00000000 as u32;
    let signed_number = unsigned_number as i32;
    println!("u32: {}, i32: {}", unsigned_number, signed_number);
    println!("u32 bits {:b}", unsigned_number);
    println!("i32 bits {:b}", signed_number);

    // as mit Typinferenz
    let a: i32;

    a = 32.0 as _;

    // ... zu bool
    let b: bool = bool::from_str("false").unwrap();

    // von und zu char
    let c = 65 as char;
    println!("{}", c);

    // Overflow, kann nicht konvertiert werden
    // let number = 0b1111111_11111111_11111111_00000000 as char;
    // println!("{}", number);

    let number = 'A' as i32;
    println!("{}", number);

    let char_option = std::char::from_u32(64);
}
