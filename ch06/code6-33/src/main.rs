fn main() {
    let x = 'C';
    let m = match x {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => -1,
    };
    println!("m is {}", m);
}
