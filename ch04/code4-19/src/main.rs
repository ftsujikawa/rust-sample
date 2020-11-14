fn main() {
    let s = "hello rust world";
    let a = &s[11..];
    println!("a is {}", a);
    let n = s.len();
    let a = &s[11..n];
    println!("a is {}", a);
}
