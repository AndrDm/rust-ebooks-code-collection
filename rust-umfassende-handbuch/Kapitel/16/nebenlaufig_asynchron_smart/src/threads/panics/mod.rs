pub fn main() {
    // Achtung, die Panic schlägt durch
    // std::thread::scope(|scope| {
    //     scope.spawn(|| {
    //         panic!("Thread 1: Panic");
    //     });
    // });

    // Die Synchronisierung eskaliert die Panic
    // std::thread::spawn(|| panic!("Thread 2: Panic")).join();
}
