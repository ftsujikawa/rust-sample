fn main() {
    let ans = test(40);
    println!("ans is {}", ans);
}
fn test(x: i32) -> i32 {
    let mut ans = x;
    if x < 0 {
        ans = 0;
    }
    if x > 100 {
        ans = 100;
    }
    ans
}