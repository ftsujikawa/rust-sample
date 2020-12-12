fn main() {
    struct Color(f32, f32, f32);
    let yello = Color(1.0,1.0,0.0);
    println!("R:{:.2} G:{:.2}  B:{:.2}", yello.0, yello.1, yello.2);
}
