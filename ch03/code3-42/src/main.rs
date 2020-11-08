fn main() {
    let num = 10;
    fn add_one(x: i32) -> i32 {
        num + x
    }
    let ans = add_one(1);
    println!("ans is {}", ans);
}
