mod interaktive_future;
mod async_await;


pub fn main() {
    #[cfg(feature = "async_await")]
    async_await::main();

    interaktive_future::main();
}