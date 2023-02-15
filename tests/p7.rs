#[path = "../problems/p7.rs"]
mod p7;
#[cfg(test)]
mod p7_tests {
    use crate::p7::Solution;

    #[test]
    fn test_case_1() {
        let x = 123;
        let expected = 321;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_2() {
        let x = -123;
        let expected = -321;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_3() {
        let x = 120;
        let expected = 21;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_4() {
        let x = 0;
        let expected = 0;
        assert_eq!(Solution::reverse(x), expected);
    }
}
