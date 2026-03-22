pub fn schreibe_modul_a() {
    println!("Modul A: {}", module_path!());
}

pub fn schreibe_anderes() {
    crate::modul_b::schreibe_modul_b();

    crate::teilbaum::modul_d::schreibe_modul_d();
}