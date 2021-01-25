#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(3, add(1,2));
    }
}
fn add(x: i32, y: i32) -> i32 {
    let ans = x + y;
    ans
}