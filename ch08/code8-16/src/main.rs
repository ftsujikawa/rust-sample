struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
impl Person {
    fn to_str(&self) -> String {
        let s = format!("{}: {} ({}) in {}", self.id, self.name, self.age, self.addr);
        s
    }
}
fn main() {
    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    let s = pa.to_str();
    println!("s is {}", s);
}
