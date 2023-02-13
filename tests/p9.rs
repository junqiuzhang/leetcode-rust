#[path = "../problems/p9.rs"]
mod p9;
#[cfg(test)]
mod tests {
    use crate::p9::Solution;

    #[test]
    fn test_case_1() {
        let x: i32 = 121;
        let expected: bool = true;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
    #[test]
    fn test_case_2() {
        let x: i32 = -121;
        let expected: bool = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
    #[test]
    fn test_case_3() {
        let x: i32 = 10;
        let expected: bool = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
}
