mod modul_a;

fn main() {
    // Fehler, modul_e ist nur ab modul_a sichtbar
    // modul_a::modul_b::modul_c::modul_d::modul_e::schreibe_e();
}
