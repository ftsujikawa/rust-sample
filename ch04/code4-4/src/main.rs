fn main() {
    let s = "hello rust world.";
    let hello = &s[0..5];
    let world = &s[11..];
    println!("hello is {}", hello);
    println!("world is {}", world);
}
