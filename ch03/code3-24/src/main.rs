fn main() {
    let x = String::from("Hello");
    let len = string_length(&x);
    println!("len is {}", len);
    println!("x is {}", x);
}

fn string_length(s: &String) -> usize {
    let length = s.len();
    length
}
