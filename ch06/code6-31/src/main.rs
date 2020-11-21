fn main() {
    let x = 3;
    let m = match x {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("m is {}", m);
}
