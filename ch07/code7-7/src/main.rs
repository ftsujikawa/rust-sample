fn main() {
    let mut ary: [u8; 16] = [0; 16];
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);
    ary[0] = 10;
    println!("ary[0] is {}", ary[0]);
    println!("ary[15] is {}", ary[15]);
}
