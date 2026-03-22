fn main() {
    println!("Wurzelmodul: {}", module_path!());

    mod untermodul_a {
        pub fn schreibe_modulname_a() {
            println!("Modulname a: {}", module_path!());
            privat();
        }

        fn privat() {
            println!("Private Funktion aufgerufen.");
        }

        pub mod untermodul_x {
            pub fn schreibe_modulname_x() {
                println!("Modulname x: {}", module_path!());
                super::privat();
            }
        }
    }

    untermodul_a::untermodul_x::schreibe_modulname_x();
    untermodul_a::schreibe_modulname_a();
}