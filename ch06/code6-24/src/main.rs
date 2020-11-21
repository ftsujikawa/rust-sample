enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
}

fn main() {
    let lang = LANG::JAPANESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
        LANG::FRANCH => "フランス語",
    };
    println!("lang is {}", m);
}
