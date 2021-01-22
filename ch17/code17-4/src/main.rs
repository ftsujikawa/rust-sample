fn main() {
    println!("use max and min");
    let a = [1,2,3,4,5];
    let max = a.iter().max();
    let min = a.iter().min();
    println!("max is {:?}", max);
    println!("min is {:?}", min);
}
