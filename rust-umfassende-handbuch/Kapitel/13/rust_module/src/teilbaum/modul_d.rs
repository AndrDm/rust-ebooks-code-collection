pub fn schreibe_modul_d() {
    println!("Modul D: {}", module_path!());
}

pub fn schreibe_anderes() {
    crate::modul_a::schreibe_modul_a();
}