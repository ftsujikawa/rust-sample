#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
fn new_person(name: &str, age: i32) -> Person {
    let p = Person {
        name: String::from(name),
        age: age,
    };
    p
}
fn main() {
    let a = new_person("masuda", 50);
    println!("a is {:?}", a);
}
