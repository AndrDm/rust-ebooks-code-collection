mod any;
mod default;
mod drop;
mod from_into;
mod kopien_und_referenzen;

pub fn main() {
    {
        // Kasten: Traits im Gültigkeitsbereich
        let mut string = String::new();

        // Holt das Trait in den Gültigkeitsbereich
        use ::std::fmt::Write;

        let _ = string.write_str("Hallo, Rust");
        println!("{string}");
    }

    any::main();
    drop::main();
    default::main();
    kopien_und_referenzen::main();
    from_into::main();
}
