pub mod modul_c;
pub mod modul_d;

pub fn schreibe_modul_a() {
    println!("Modul A: {}", module_path!());
}