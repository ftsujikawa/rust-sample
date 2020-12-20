#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
fn new_person(name: &str, id: i32) -> &Person {
    let p = Person {
        id: id,
        name: String::from(name),
        age: -1,
        addr: String::from("Unknown"),
    };
    &p
}
fn main() {
    let a = new_person("masuda", 50);
    println!("a is {:?}", a);
}
