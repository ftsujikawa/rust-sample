fn main() {
    let mut s = String::new();
    str_param_complex(&mut s);
    println!("s is {}", s);
}
fn str_param_complex(s: &mut String) {
    *s = String::from("hello");
}