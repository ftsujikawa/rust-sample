use std::thread;
use std::time::Duration;
fn task() {
    for i in 0..10 {
        println!("thread #1 count {}.", i);
        thread::sleep(Duration::from_millis(1000));
    }
}
fn main() {
    let handle = thread::spawn(task);
    println!("please wait.");
    handle.join().unwrap();
    println!("program end.");
}
