struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

fn main() {
    let mut pa = Person {
        id: 100,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
    pa.age += 1;
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
    pa.addr = String::from("Osaka");
    println!("{}: {} ({}) in {}", pa.id, pa.name, pa.age, pa.addr);
}
