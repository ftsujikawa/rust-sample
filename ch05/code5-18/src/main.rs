fn main() {
    let ret = two_param_and_return(10, 20);
    println!("ret is {}", ret);
}
fn two_param_and_return(x: i32, y: i32) -> i32 {
    println!("called two_param_and_return, {} and {}", x, y);
    x + y
}