fn main() {
    let print_name = |name, age| {
        println!("name: {}, age: {}", name, age);
    };
    println!("call closure");
    print_name("masuda", 50);
}
