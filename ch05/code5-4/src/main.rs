fn main() {
    let a = 10 + 20;
    println!("a is {}", a);
    let a = { 10 + 20 };
    println!("a is {}", a);
}
