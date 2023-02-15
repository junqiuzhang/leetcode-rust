#[path = "../problems/p7.rs"]
mod p7;
#[cfg(test)]
mod p7_tests {
    use crate::p7::Solution;

    #[test]
    fn test_case_1() {
        let x: i32 = 123;
        let expected: i32 = 321;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_2() {
        let x: i32 = -123;
        let expected: i32 = -321;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_3() {
        let x: i32 = 120;
        let expected: i32 = 21;
        assert_eq!(Solution::reverse(x), expected);
    }
    #[test]
    fn test_case_4() {
        let x: i32 = 0;
        let expected: i32 = 0;
        assert_eq!(Solution::reverse(x), expected);
    }
}
