fn main() {
    let mut v = vec![1,2,3,4,5];
    println!("v.first is {}", v.first().unwrap());
    println!("v.last is {}", v.last().unwrap());
}
