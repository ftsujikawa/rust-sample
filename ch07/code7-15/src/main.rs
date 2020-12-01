fn main() {
    let mut v = vec![1,2,3,4,5];
    println!("v.first is {:?}", v.first());
    println!("v.last is {:?}", v.last());
    println!("v.get(1) is {:?}", v.get(1));
    println!("v.get(10) is {:?}", v.get(10));
}
