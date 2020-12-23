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
    println!("x is {:?}", x);
    x.name = String::from("kato");
    x.age = 0;
    println!("a is {:?}", a);
    println!("x is {:?}", x);
}
