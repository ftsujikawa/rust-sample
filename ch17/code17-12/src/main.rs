fn call_with_one<F>(x: usize, func: F) -> usize
    where F: Fn(usize) -> usize {
        func(x)
    }
fn main() {
    let double = |x| x * 2;
    let triple = |x| x * 3;
    let a = call_with_one(100, double);
    let b = call_with_one(100, triple);
    println!("a is {}", a);
    println!("b is {}", b);
}
