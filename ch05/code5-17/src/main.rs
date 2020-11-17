fn main() {
    one_param(10);
    two_param(10, 20);
}
fn one_param(x: i32) {
    println!("called one_param, x is {}", x);
}
fn two_param(x: i32, y: i32) {
    println!("called two_param, {} and {}", x, y);
}