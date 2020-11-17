fn main() {
    let ret = str_param_and_return("rust");
    println!("ret is {}", ret);
}
fn str_param_and_return(s: &str) -> String {
    println!("called str_param_and_return, s is {}", s);
    let ret = format!("hello {} world.", s);
    ret
}