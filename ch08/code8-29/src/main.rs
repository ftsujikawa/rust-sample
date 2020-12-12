use std::num::ParseIntError;
type Result<T> = std::result::Result<T, ParseIntError>;
fn half_number(s: &str) -> Result<i32> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}
fn main() {
    match half_number("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
