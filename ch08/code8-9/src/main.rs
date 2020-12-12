struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
fn add_age(pa: &mut Person) {
    pa.age += 1;
}
fn print_person(pa: &Person) {
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}
fn main() {
    let mut pa = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    print_person(&pa);
    add_age(&pa);
    print_person(&pa);
}