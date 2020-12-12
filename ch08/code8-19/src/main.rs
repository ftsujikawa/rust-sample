static mut PERSON_ID: i32 = 0;
struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
    fn print(&self) {
        println!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
    }
}
fn main() {
    let mut people = Vec::<Person>::new();
    people.push(Person::new("masuda", 50, "Tokyo"));
    people.push(Person::new("kato", 30, "Osaka"));
    people.push(Person::new("yamada", -1, "unknown"));
    people.push(Person::new("sato", -1, "unknown"));
    for p in &people {
        p.print();
    }
}
