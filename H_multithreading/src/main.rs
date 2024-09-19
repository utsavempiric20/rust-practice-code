use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawn {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap();

    for i in 0..5 {
        println!("main {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}
