fn main() {
    let mut v: Vec<i32> = Vec::new();
    println!("v.len is {}", v.len());
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v.len is {}", v.len());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("pop is {:?}", v.pop());
    println!("v.len is {}", v.len());
}
