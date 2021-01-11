fn main() {
    sub();
}
fn sub() {
    subsub();
}
fn subsub() {
    panic!("このプログラムは動きません");
}