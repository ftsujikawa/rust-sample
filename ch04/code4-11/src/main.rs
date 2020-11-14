fn main() {
    let s = "こんにちは rust コードの世界";
    let hello = &s[0..15];
    let world = &s[21..];
    println!("こんにちは is {}", hello);
    println!("コードの世界 is {}", world);
}
