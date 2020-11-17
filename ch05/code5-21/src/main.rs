fn main() {
    str_param("rust");
}
fn str_param(s: &str) {
    s = "change rust";
    println!("called str_param, s is {}", s);
}