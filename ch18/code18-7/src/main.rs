use std::thread;
use std::time::Duration;
fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}
fn main() {
    println!("program start.");
    let h1 = thread::spawn(|| {foo(10);});
    let h2 = thread::spawn(|| {foo(20);});
    let h3 = thread::spawn(|| {foo(30);});
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    println!("program end.");
}
