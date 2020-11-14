fn main() {
    let s = "こんにちは rust コードの世界";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);
}
