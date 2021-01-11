use std::path::Path;
fn main() {
    let path = "unknown.txt";
    if Path::new(path).exists() {
        println!("ファイルが存在する");
    }
    else {
        panic!("ファイルが存在しない");
    }
}
