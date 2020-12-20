fn main() {
    let a = String::from("masuda");
    println!("a is {}", a);
    let x = &a;
    println!("x is {}", x);
    println!("a is {}", a);
}
