fn main() {
    let n = "100".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n);
    let n = "xxx".parse::<i32>().expect("これは数値ではありません");
    println!("n is {}", n);
}
