fn main() {
    let a = vec!["one","two","three"];
    println!("a[] is {:?}", a);
    let x = &a;
    println!("x[] is {:?}", x);
    println!("a[] is {:?}", a);
}
