mod shadowing_ohne;
mod shadowing_mit;

pub fn main() {
    let alter = 42;
    // Eine Referenz in einen Zeiger beugen
    let erstes_alter: *const i32 = &alter;
    let alter = 21;
    println!("{alter}"); // 21

    unsafe {
        // "42" liegt weiterhin an der Adresse im Stack
        println!("Erstes Alter: {}", erstes_alter.read());
    }

    shadowing_ohne::main();
    shadowing_mit::main();
}