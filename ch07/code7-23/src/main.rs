fn main() {
    let v = vec![1,2,3,4,5];
    print!("for is ");
    for i in &v {
        print!("{} ", i);
    }
    println!("");
    print!("for and iter is ");
    for i in v.iter() {
        print!("{} ", i);
    }
    println!("");
}
