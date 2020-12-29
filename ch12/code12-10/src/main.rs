use std::fs::File;
use std::io::Read;

fn main() {
    let path = "sample.txt";
    println!("read 16 bytes by buffer.");
    let mut file = File::open(path).expect("file not found");
    let mut buf: [u8; 1] = [0; 1];
    for i in 0..16 {
        file.read(&mut buf);
        println!("buf is {}: {}", i, buf[0] as char);
    }
}
