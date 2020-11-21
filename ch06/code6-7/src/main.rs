fn main() {
    let a = 10;
    let b = 20;
    if test(a, b) {
        println!("test is ok.");
    }
}
fn test(x: i32, y: i32) -> bool {
    x < y
}