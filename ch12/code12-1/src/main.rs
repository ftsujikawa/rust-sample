fn main() {
    let path = "sample.txt";
    println!("read all lines.");
    if let Ok(data) = std::fs::read_to_string(path) {
        println!("data is {}", data);
    }
}
