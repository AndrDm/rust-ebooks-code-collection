mod modul_a {
    pub fn schreibe_modulname_a() {
        println!("Modulname a: {}", module_path!());
    }
}

fn main() {
    println!("Wurzelmodul: {}", module_path!());

    modul_a::schreibe_modulname_a();
}