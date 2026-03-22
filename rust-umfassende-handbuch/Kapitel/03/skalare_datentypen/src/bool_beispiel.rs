pub fn main() {
    let is_first_bool = true;
    let byte_size = std::mem::size_of_val(&is_first_bool);
    let numeric_value = i8::from(is_first_bool);

    println!(
        "Anzahl Bytes: {}, \
         Bitmuster: {:#X}",
        byte_size, numeric_value
    );
}