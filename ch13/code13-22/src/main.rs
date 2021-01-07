fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("パラメータは必要です");
    }
    else {
        for (i, s) in args.iter().enumerate() {
            println!("{}: {}", i, s);
        }
    }
}
