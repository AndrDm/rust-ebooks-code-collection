mod explizite_diskriminanten;
mod implizite_diskriminante;
mod muster;
mod speicherbedarf;
mod struktur_variante;
mod tupel_variante;
mod variante_datentyp;
mod verhalten;

fn main() {
    implizite_diskriminante::main();
    explizite_diskriminanten::main();
    tupel_variante::main();
    struktur_variante::main();
    variante_datentyp::main();
    speicherbedarf::main();
    muster::main();
    verhalten::main();
}
