fn main() {
    let ans = test(40);
    println!("ans is {}", ans);
}
fn test(x: i32) -> i32 {
    if x < 0 {
        0
    }
    else if x > 100 {
        100
    }
    else {
        x
    }
}