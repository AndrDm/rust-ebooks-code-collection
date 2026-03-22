#[path = "module/modul_c.rs"]
pub mod modul_c;

#[path = "module/modul_d.rs"]
pub mod modul_d;

pub mod modul_a;
pub mod modul_b;

fn main() {
    println!("Modulname: {}", module_path!());
    modul_a::schreibe_modul_a();
    modul_b::schreibe_modul_b();
    // Jetzt auf einer Ebene mit "module_a" und "module_b"
    modul_c::schreibe_modul_c();
    modul_d::schreibe_modul_d();
}
