fn main() {
    let a = [1u8,2u8,3u8,4u8];
    unsafe {
        let b = std::mem::transmute::<[u8; 4], u32>(a);
        println!("b is {:X}", b);
    }
    let a: u32 = 0x11223344;
    unsafe {
        let b = std::mem::transmute::<u32, [u8; 4]>(a);
        println!("b[] is {:X} {:X} {:X} {:X}", b[0], b[1], b[2], b[3]);
    }
}
