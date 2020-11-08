fn main() {
    let name = get_person_name();
    let age = get_person_age();

    if age > 50 {
        age = 50;
    }

    println!("name si {}, age {}", name, age);
}
fn get_person_name() -> String {
    "Hello".to_string()
}
fn get_person_age() -> i32 {
    28
}