fn main() {
    println!("use return of closure");
    let format_name = |name, age| {
        format!("name: {}, age: {}", name, age)
    };
    let x = format_name("kato", 40);
    println!("x is {}", x);
}
