use std::num::ParseIntError;

fn half_number(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err(err),
    }
}
fn main() {
    match half_number("100") {
        Ok(n) => println!("Ok is {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}