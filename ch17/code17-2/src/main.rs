fn main() {
    println!("use filter");
    let a = [1,2,3,4,5];
    let b = a.iter().filter(|&x| x % 2 == 1);
    for it in b {
        println!("it is {}", it);
    }
}
