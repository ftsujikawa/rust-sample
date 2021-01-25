fn main() {
    println!("Hello, Rust world!");
    let mut a = Person { id: 100, name: "masuda".to_string(), age: 50 };
    let b = Person { id: 100, name: "masuda".to_string(), age: 50 };
    let c = Person { id: 200, name: "kato".to_string(), age: 40 };
    assert_eq!("masuda", a.name);
    assert_eq!("kato", c.name);
    assert_eq!("unknown", c.name);
    println!("a is {:?}", a);
    println!("c is {:?}", c);
    assert_eq!(a, b);
    a.age += 1;
    println!("a is {:?}", a);
    assert_ne!(a, b);
}
#[derive(Debug,PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32
}