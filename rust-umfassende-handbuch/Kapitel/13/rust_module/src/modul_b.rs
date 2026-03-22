pub fn schreibe_modul_b() {
    println!("Modul B: {}", module_path!());
}

pub fn schreibe_anderes() {
    crate::modul_a::schreibe_modul_a();
}