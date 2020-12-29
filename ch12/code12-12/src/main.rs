use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let path = "sample.txt";
    let file = File::open(path)?;
    for line in BufReader::new(file).lines() {
        println!("line is {}", line?);
    }
    Ok(())
}
