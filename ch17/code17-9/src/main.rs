fn main() {
    let print_name = |name: String, age: i32| {
        println!("name: {}, age: {}", name, age);
    };
    println!("call closure");
    print_name("masuda", 50);
}
