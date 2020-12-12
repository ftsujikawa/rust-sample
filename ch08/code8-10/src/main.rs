struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
fn new_person(id: i32, name: &str) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}
fn main() {
    let pa2 = new_person(200, "kato");
    println!("{}: {} ({}) in {}", pa2.id, pa2.name, pa2.age, pa2.addr);
}
