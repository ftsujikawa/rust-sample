trait ToNumber {
    fn to_i(&self) -> i32;
}
impl ToNumber for String {
    fn to_i(&self) -> i32 {
        match self.parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        }
    }
}
fn main() {
    println!("Hello, world!");
}
