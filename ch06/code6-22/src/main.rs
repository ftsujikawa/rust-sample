#[derive(Debug)]
enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
}
fn main() {
    let lang = LANG::JAPANESE;
    println!("lang is {:?}", lang);
}
