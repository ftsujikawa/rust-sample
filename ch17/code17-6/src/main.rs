fn main() {
    println!("use zip");
    let a = [1,2,3,4,5];
    let b = ["one", "two", "three", "four", "five"];
    let c: Vec<_> = a.iter().zip(b.iter()).collect();
    for (n,w) in c {
        println!("c is {} and {}", n, w);
    }
}
