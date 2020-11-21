enum LANG {
    JAPANESE,
    ENGLISH,
    CHINESE,
    FRANCH,
}

fn main() {
    let lang = LANG::CHINESE;
    let m = match lang {
        LANG::JAPANESE => "日本語",
        LANG::ENGLISH => "英語",
        LANG::CHINESE => "中国語",
    };
    println!("lang is {}", m);
}
