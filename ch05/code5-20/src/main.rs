fn main() {
    str_param("rust");
}
fn str_param(s: &str) {
    println!("called str_param, s is {}", s);
}