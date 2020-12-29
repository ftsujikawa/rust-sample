use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "sample.txt";
    println!("read every one line.");
    let file = File::open(path).expect("file not found.");
    for line in BufReader::new(file).lines() {
        if let Ok(l) = line {
            println!("line is {}", l)
        }
    }
}
