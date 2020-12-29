use std::fs::File;
use std::io::Write;

fn main() {
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found");
    let s = "hello rust world.\n";
    file.write(s.as_bytes()).expect("cannot write");
}
