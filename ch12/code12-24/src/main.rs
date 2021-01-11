use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        //
    }
    else {
        let file = File::open(&args[1]).expect("file not found.");
        let reader = BufReader::new(file);
        let mut writer = BufWriter::new(std::io::stdout());
        for it in reader.bytes() {
            writer.write(&[it.unwrap()]);
        }
    }
}
