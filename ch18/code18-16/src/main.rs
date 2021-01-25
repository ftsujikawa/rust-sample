use std::thread;
use std::time::Duration;
async fn foo(id: i32) {
    for i in 0..10 {
        println!("thread #{} count {}.", id, i);
        thread::sleep(Duration::from_millis(1000));
    }
}
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    println!("program start.");
    rt.block_on(async {
        foo(10).await;
        foo(20).await;
        foo(30).await;
    });
    println!("program end.");
}
