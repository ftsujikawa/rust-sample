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
    thread::spawn(|| {foo(10);}).join().unwrap();
    thread::spawn(|| {foo(20);}).join().unwrap();
    thread::spawn(|| {foo(30);}).join().unwrap();
    println!("program end.");
}
