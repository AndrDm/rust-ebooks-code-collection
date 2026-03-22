use std::thread;
use std::time::Duration;

pub fn main() {
    thread::scope(|scope| {
        let handle = scope.spawn(|| {
            let mut i = 0;
            // for mut i in 0.. {
            loop {
                println!("Zaehler: {i}");
                if i >= 5 {
                    i = 0;
                    println!("Parke!");
                    thread::park();
                }
                i += 1;
                thread::sleep(Duration::from_millis(400));
            }
        });

        scope.spawn(move || loop {
            thread::sleep(Duration::from_secs(4));
            handle.thread().unpark();
            println!("Ausparken!");
        });
    });
}
