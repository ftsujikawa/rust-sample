fn main() {
    let x = Some(10);
    let v = match x {
        Some(i) => i,
        None => -1,
    };
    println!("v is {}" ,v);
}
