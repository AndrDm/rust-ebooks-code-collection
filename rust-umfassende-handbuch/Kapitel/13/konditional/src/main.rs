mod konfiguration;
mod makro;
mod debug_assertions;

fn main() {
    konfiguration::main();
    makro::main();
    debug_assertions::main();
}
