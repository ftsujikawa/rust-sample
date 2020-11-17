fn main() {
    let a = 10;
    if plus(a) {
        println!("plus(a) is {}", a);
    }
}
fn plus(x: i32) -> bool {
    x > 0
}