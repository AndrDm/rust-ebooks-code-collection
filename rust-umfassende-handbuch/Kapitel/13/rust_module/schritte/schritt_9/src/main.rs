mod module {
    mod werkzeuge {
        mod modul_b;
    }

    mod services {
        // ...
    }
}

fn main() {
    println!("Modulname: {}", module_path!());
}
