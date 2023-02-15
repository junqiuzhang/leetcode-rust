#[path = "../problems/p9.rs"]
mod p9;
#[cfg(test)]
mod p9_tests {
    use crate::p9::Solution;

    #[test]
    fn test_case_1() {
        let x = 121;
        let expected = true;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
    #[test]
    fn test_case_2() {
        let x = -121;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
    #[test]
    fn test_case_3() {
        let x = 10;
        let expected = false;
        assert_eq!(Solution::is_palindrome(x), expected);
    }
}
