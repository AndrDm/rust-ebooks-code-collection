pub fn main() {
    let mut builder = std::thread::Builder::new();
    let thread = builder
        .name("Neuer Thread".into())
        .stack_size(1024)
        .spawn(|| {
            // ...
        });
    // ...

    let _ = thread.unwrap().join();
}
