fn main() {
    let ans = test(40);
}
fn test(x: i32) -> i32 {
    if x < 0 {
        let ans = 0;
    }
    if x > 100 {
        let ans = 100;
    }
    ans
}