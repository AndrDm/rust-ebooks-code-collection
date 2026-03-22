mod modul_a;
mod modul_b;
mod teilbaum;

fn main() {
    println!("Modulname: {}", module_path!());

    modul_a::schreibe_modul_a();
    modul_a::schreibe_anderes();

    modul_b::schreibe_modul_b();
    modul_b::schreibe_anderes();
}
