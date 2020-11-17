fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = vec_param(&v);
    println!("sum is {}", sum);
}
fn vec_param(v: &Vec<i32>) -> i32 {
    println!("called vec_param");
    let mut sum = 0;
    for i in v {
        sum += i;
    }
    sum
}