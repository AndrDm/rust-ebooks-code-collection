mod deklarativ;

fn main() {
    #[cfg(feature = "deklarativ")]
    deklarativ::main();
}
