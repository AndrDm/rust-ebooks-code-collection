mod modul_e;

mod modul_a {
    pub fn schreibe_modul_a() {
        println!("Modul A: {}", module_path!());
    }

    pub mod modul_c {
        pub fn schreibe_modul_c() {
            println!("Modul C: {}", module_path!());
        }
    }

    pub mod modul_d {
        pub fn schreibe_modul_d() {
            println!("Modul D: {}", module_path!());
        }
    }
}

mod modul_b {
    pub fn schreibe_modul_b() {
        println!("Modul B: {}", module_path!());
    }

    fn schreibe_andere() {
        super::modul_a::modul_c::schreibe_modul_c();
        super::modul_a::modul_d::schreibe_modul_d();
    }
}

fn main() {
    println!("Modulname: {}", module_path!());

    modul_a::schreibe_modul_a();
    modul_b::schreibe_modul_b();
    modul_a::modul_c::schreibe_modul_c();
    modul_a::modul_d::schreibe_modul_d();
}
