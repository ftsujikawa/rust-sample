use std::fs::File;
use std::io::Write;

fn main() {
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found");
    file.write(b"hello rust world.\n").expect("cannot write");
}
