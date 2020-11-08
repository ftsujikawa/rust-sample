fn main() {
    let ans = add_two(10, 20);
    println!("ans is {}", ans);
    let ans = add_one(30);
    println!("ans is {}", ans);
}
fn add_two(x: i32, y: i32) -> i32 {
    x + y
}
fn add_one(x: i32) -> i32 {
    x + 1
}