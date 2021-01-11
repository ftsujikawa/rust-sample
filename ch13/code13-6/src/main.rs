fn main() {
    let n = Option::<i32>::Some(10);
    println!("option is {:?}.", n);
    let n = Option::<i32>::None;
    println!("option is {:?}.", n);
}
