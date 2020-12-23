#[derive(Debug)]
struct Person {
    name: &'static str,
    age: i32
}
fn main() {
    let x = Person {
        name: "tsu",
        age: 54,
    };
    println!("x is {:?}", x);
}