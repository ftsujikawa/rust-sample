fn main() {
    let s = "hello rust world";
    let a = &s[6..10];
    println!("a is {}", a);
    let a = &s[6..(6+4)];
    println!("a is {}", a);
}
