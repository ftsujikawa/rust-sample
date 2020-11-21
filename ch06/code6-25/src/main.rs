enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
}

fn main() {
    let lang = LANG::ENGLISH;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        _ => "日本語以外"
    };
    println!("lang is {}", m);
}
