mod modul_a;

pub fn schreibe_modul_b() {
    println!("Modul B: {}", module_path!());
}

fn main() {
    schreibe_modul_b();
}
