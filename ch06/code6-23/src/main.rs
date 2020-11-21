enum LANG {
    JAPANESE = 81,
    ENGLISH = 44,
    CHINESE = 86,
    FRANCH = 33,
}

fn main() {
    let lang = LANG::JAPANESE;
    println!("lang is {}", lang as i32);
}
