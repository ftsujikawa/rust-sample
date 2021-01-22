fn main() {
    println!("use max and min");
    let a: [i32;0] = [];
    let max = a.iter().max();
    let min = a.iter().min();
    println!("max is {:?}", max);
    println!("min is {:?}", min);
}
