fn half_number(s: &str) -> i32 {
    s.parse::<i32>().unwrap() / 2
}
fn main() {
    let n: i32 = half_number("xxx");
    println!("n is {}", n);
}
