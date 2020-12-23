#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
fn main() {
    let a = new_person("masuda", 50);
    println!("a is {:?}", a);
}
