use std::fs::File;
use std::io::Write;

fn main() {
    let path = "sample.txt";
    let mut file = File::create(path).expect("file not found.");
    let s = "hello rust world.\n";
    for it in s.as_bytes() {
        let ch = *it;
        let ary = [ch];
        file.write(&ary).expect("cannot write.");
    }
}
