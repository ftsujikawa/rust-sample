fn main() {
    println!("use map");
    let a = [("masuda", 50), ("kato", 40), ("yamada", 30)];
    let b = a.iter().map(|(name,age)| {
        format!("name: {}, age: {}", name, age)
    });
    for it in b {
        println!("{}", it);
    }
}
