fn main() {
    let x = 5;
    let m = match x {
        0..=5 => "5以下",
        0..=10 => "6以上10以下",
        _ => "10より大きい",
    };
    println!("m is {}", m);
}
