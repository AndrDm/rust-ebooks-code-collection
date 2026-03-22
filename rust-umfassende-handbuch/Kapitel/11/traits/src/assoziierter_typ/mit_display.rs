use std::fmt::Display;

trait Form {
    type Koordinate: Display;

    fn ausgeben(k: Self::Koordinate) {
        println!("Eine Koordinate: {}", k);
    }
}
