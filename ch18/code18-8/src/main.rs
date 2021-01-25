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
    h1.join().unwrap();
    let h2 = thread::spawn(|| {foo(20);});
    h2.join().unwrap();
    let h3 = thread::spawn(|| {foo(30);});
    h3.join().unwrap();
    println!("program end.");
}
