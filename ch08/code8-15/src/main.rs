struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
impl Person {
    fn print_t(&self, private: bool) {
        if private == true {
            println!("{}: {}", self.id, self.name);
        }
        else {
            println!("{}: {}({}) in {}", self.id, self.name, self.age, self.addr);
        }
    }
}
fn main() {
    let pa = Person {
        id: 1,
        name: String::from("masuda"),
        age: 50,
        addr: String::from("Tokyo"),
    };
    pa.print_t(true);
    pa.print_t(false);
}
