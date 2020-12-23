#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32
}
fn move_a(a: Person) {
    println!("print_a: a is {:?}", a);
}
fn main() {
    let a = Person { name: "masuda", age: 50 };
    move_a(a);
    println!("main: a is {:?}", a);
}