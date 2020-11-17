fn main() {
    let a = add(10, 20);
    println!("a is {}", a);
}
fn add(x: i32, y: i32) -> i32 {
    x + y
}