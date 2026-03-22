fn main() {

}


#[cfg(feature = "feature_a")]
mod feature_a {
    // ...
}

#[cfg(feature = "feature_b")]
mod feature_b {
    // ...
}

#[cfg(feature = "feature_c")]
struct C;

#[cfg(feature = "feature_d")]
struct D;

#[cfg(feature = "feature_e")]
mod modul_e {
    // ...
}