use std::thread;
use std::time::Duration;

fn main() {
    let h = thread::spawn(||{
        for i in 0..20{
            println!("\tSou a thread. Contador = {i}");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 0..10 {
        println!("I'm the main Thread! Count is {i}");
        thread::sleep(Duration::from_secs(1));
    }

    h.join().unwrap();
}

