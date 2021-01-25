use std::thread;
use std::time::Duration;
use std::io::Read;
fn main() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("thread #1 count {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });
    println!("press enter key.");
    std::io::stdin().read(&mut [0]);
    println!("program end.");
}
