#[derive(Debug,PartialEq)]
struct Person {
    id: i32,
    name: String,
    age: i32
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equal_instance() {
        let mut a = Person { id: 100, name: "masuda".to_string(), age: 50 };
        let b = Person { id: 100, name: "masuda".to_string(), age: 50 };
        let c = Person { id: 200, name: "kato".to_string(), age: 40 };
        assert_eq!(a, a);
        assert_eq!(a, b);
        assert_ne!(a, c);
        a.age += 1;
        assert_ne!(a, b);
        let x = &a;
        assert_eq!(a, *x);
    }
}
