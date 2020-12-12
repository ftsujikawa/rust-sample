fn half_number(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err("実行エラー：これは数値ではありません"),
    }
}
fn main() {
    match half_number("100") {
        Ok(n) => println!("n is {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
    match half_number("xxx") {
        Ok(n) => println!("n is {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}
