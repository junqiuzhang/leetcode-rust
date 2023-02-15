#[path = "../problems/p1.rs"]
mod p1;
#[cfg(test)]
mod p1_tests {
    use crate::p1::Solution;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
