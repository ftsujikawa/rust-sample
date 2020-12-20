#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
fn main() {
    let mut a = Person {
        name: String::from("masuda"),
        age: 50,
    };
    println!("a is {:?}", a);
    let mut x = &mut a;
    x.age = 0;
    println!("x is {:?}", x);
    let mut y = &mut a;
    y.name = String::from("kato");
    println!("y is {:?}", y);
    println!("a is {:?}", a);
}
