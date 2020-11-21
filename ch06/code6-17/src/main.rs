fn main() {
    print!("FOR is ");
    for i in 0..10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!("");
}
