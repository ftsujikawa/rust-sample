fn main() {
    let v = [10,20,30,40,50];
    print_i32(&v);
    let v = ['A','B','C','D','E'];
    print_char(&v);
    let v = ["one","two","three","four","five"];
    print_str(&v);
}
fn print_i32(a: &[i32]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}
fn print_char(a: &[char]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}
fn print_str(a: &[&str]) {
    print!("a is ");
    for i in a {
        print!("{} ", i);
    }
    println!("");
}