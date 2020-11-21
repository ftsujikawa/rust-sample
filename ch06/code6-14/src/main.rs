fn main() {
    let v = vec![10,20,30,40,50];
    print!("v is ");
    for (i, x) in v.iter().enumerate() {
        print!("{}:{} ", i, x);
    }
    println!("");
}
