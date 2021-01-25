#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1,2));
        assert_eq!(10, add(0,10));
        assert_eq!(10, add(10,0));
        assert_eq!(20, add(10,10));
    }
}
fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    ans
}