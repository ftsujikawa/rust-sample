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
    println!("a is {:?}", a);
    add_age(&mut a);
    println!("a is {:?}", a);
}