struct Rectange {
    width: f32,
    height: f32,
}
struct Triangle {
    base: f32,
    height: f32,
}
struct Circle {
    radius: f32,
}
trait CalcArea {
    fn calc_area(&self) -> f32;
}
trait ExprString {
    fn expr_str(&self) -> String {
        "幅　✕　高さ　＝　".to_string()
    }
}
impl ExprString for Rectange {}
impl ExprString for Triangle {
    fn expr_str(&self) -> String {
        "底辺　✕　高さ　÷　２　＝　".to_string()
    }
}
impl ExprString for Circle {
    fn expr_str(&self) -> String {
        "π　✕　半径　✕　半径　＝　".to_string()
    }
}
fn main() {
    println!("Hello, world!");
}
