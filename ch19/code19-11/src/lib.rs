#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_not_equal() {
        assert_ne!(0, 1+1);
        assert_ne!("Javascript", "Java");
    }
}
