use std::fs;

fn main() {
    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = fs::read_to_string(path) {
        println!("data is {}", data);
    }
}
