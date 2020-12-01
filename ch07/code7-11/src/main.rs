fn main() {
    let v = vec![1,2,3,4,5];
    println!("v.get(0) is {:?}", v.get(0));
    println!("v.get(4) is {:?}", v.get(4));
    println!("v.get(0) is {}", v.get(0).unwrap());
    println!("v.get(4) is {}", v.get(4).unwrap());
}
