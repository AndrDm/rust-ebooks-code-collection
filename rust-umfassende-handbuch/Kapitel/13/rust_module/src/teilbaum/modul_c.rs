pub fn schreibe_modul_c() {
    println!("Modul C: {}", module_path!());
}

pub fn schreibe_anderes() {
    crate::modul_a::schreibe_modul_a();
}