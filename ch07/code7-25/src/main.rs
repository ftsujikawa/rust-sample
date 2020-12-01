fn main() {
    let v = vec![1,2,3,4,5];
    println!("by loop");
    let mut p = v.iter();
    loop {
        let x = p.next();
        if x == None {
            break;
        }
        println!("x is {}", x.unwrap());
    }
    println!("by while");
    let mut p = v.iter();
    while let Some(x) = p.next() {
        println!("x is {}", x);
    }
}
