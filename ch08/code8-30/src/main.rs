use std::num::ParseIntError;
fn half_number(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(|n| n / 2)
}
fn main() {
    match half_number("100") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
