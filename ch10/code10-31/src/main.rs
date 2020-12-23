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
    let mut y = &mut a;
    x.age = 0;
    y.name = String::from("kato");
    println!("a is {:?}", a);
    println!("x is {:?}", x);
    println!("y is {:?}", y);
}
