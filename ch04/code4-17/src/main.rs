fn main() {
    let s = "hello rust world";
    let a = &s[0..1];
    println!("a is {}", a);
    let a = &s[0..5];
    println!("a is {}", a);
    let a = &s[..5];
    println!("a is {}", a);
}
