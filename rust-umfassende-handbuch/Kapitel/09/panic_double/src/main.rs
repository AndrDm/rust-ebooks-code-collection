fn main() {
    std::panic::set_hook(
        Box::new(
            |info| {
                let ungerade_zahl = info
                    .payload()
                    .downcast_ref::<String>()
                    .unwrap();

                println!(
                    "Eine Panic ist aufgetreten: {ungerade_zahl}"
                );
            }
        )
    );

    panic!("Eine 'unerwartete' Panic");
}
