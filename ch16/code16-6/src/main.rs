#[link(name="hello", kind="static")]
extern {
    fn c_add(a: i32, b: i32) -> i32;
}
fn main() {
    let a = 10;
    let b = 20;
    let ans = unsafe {
        c_add(a, b)
    };
    println!("ans is {}", ans)
}