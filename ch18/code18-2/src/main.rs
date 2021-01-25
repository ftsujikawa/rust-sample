use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}.", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("please wait.");
    handle.join().unwrap();
    println!("program end.");
}
