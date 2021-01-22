fn main() {
    let print_name = |name: &str, age: i32| {
        println!("name: {}, age: {}", name, age);
    };
    println!("call closure");
    print_name("masuda", 50);
}
