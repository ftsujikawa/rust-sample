#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equal() {
        assert_eq!(2, 1+1);
        assert_eq!(1.123, 1.0 + 0.123);
        assert_eq!(true, 1 == 1);
        assert_eq!("rust", " rust ".trim());
    }
}
