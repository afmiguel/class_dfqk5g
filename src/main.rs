use std::thread;
use std::time::Duration;

fn main() {
    for i in 0..10 {
        println!("I'm the main Thread! Count is {i}");
        thread::sleep(Duration::from_secs(1));
    }
}

