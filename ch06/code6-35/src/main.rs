fn main() {
    let x = "three";
    let m = match x {
        "one" => 1,
        2 => 2,
        "three" => 3,
        _ => -1,
    };
    println!("m is {}", m);
}
