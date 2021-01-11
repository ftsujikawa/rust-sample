use std::fs;
fn main() {
    let path = "unknown.txt";
    println!("read all lines.");
    match fs::read_to_string(path) {
        Ok(data) => { println!("data is {}", data); },
        _ => { println!("cannot open {}", path); }
    }
}
