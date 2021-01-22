fn call_with_vec<F>(v : &Vec::<usize>, func: F) -> usize
    where F: Fn(usize) -> usize {
        let mut sum = 0;
        for it in v {
            sum += func(*it);
        }
        sum
    }
fn main() {
    let double = |x| x * 2;
    let v = vec![1,2,3,4,5];
    let a = call_with_vec(&v, double);
    println!("a is {}", a);
    let sum: usize = v.iter().map(|x| x * 2).sum();
    println!("sum is {}", sum);
}
