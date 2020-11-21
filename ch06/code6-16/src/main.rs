fn main() {
    print!("FOR is ");
    for i in 0..5 {
        if i == 5 {
            break;
        }
        print!("{} ", i);
    }
    println!("");
}
