fn main() {
    println!("A size_of is {}", std::mem::size_of::<A>());
    println!("B size_of is {}", std::mem::size_of::<B>());
    println!("C size_of is {}", std::mem::size_of::<C>());
    println!("D size_of is {}", std::mem::size_of::<D>());
    println!("E size_of is {}", std::mem::size_of::<E>());
    println!("F size_of is {}", std::mem::size_of::<F>());
}
struct A {
    id: i32,
}
struct B {
    id1: i32,
    id2: i32,
    id3: i32,
}
struct C {
    id: i32,
    name: String,
}
struct D {
    id: i32,
    name: &'static str,
}
struct E {
    id: i32,
    v: Vec<u8>,
}
struct F {
    id: i32,
    v: [u8; 100],
}