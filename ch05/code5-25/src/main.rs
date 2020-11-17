fn main() {
    let v = vec_return(10);
    for i in v {
        print!("{} ", i);
    }
    println!("");
}
fn vec_return(max: i32) -> Vec<i32> {
    println!("called vec_return");
    let mut v = Vec::new();
    for i in 0..max {
        v.push(i);
    }
    v
}