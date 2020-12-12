struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
impl Person {
    fn print(&self) {
        println!("{}: {}({}) in {}", self.id, self.name, self.age, self.addr);
    }
    fn add_age(&self, n: i32) {
        self.age += n;
    }
}
fn main() {
    let mut pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print();
    pa.add_age(1);
    pa.print();
}
