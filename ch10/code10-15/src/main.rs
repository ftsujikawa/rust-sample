#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32
}
fn add_age(a: &mut Person) {
    a.age += 1;
}
fn main() {
    let mut a = Person { name: "masuda", age: 50 };
    let mut x = &mut a;
    println!("x is {:?}", x);
    x.age += 1;
    println!("x is {:?}", x);
    add_age(&mut x);
    println!("x is {:?}", x);
}