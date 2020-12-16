fn print<T>(a: &[T]) where T: std::fmt::Debug {
    print!("a is ");
    for i in a {
        print!("{:?} ", i);
    }
    println!("");
}
fn main() {
    let a = vec![1,2,3,4,5];
    print(&a);
}