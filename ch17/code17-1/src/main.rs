fn main() {
    println!("use map");
    let a = [1,2,3,4,5];
    let b = a.iter().map(|x| x*x);
    for it in b {
        println!("it is {}", it);
    }
}
