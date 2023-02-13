#[path = "../problems/p1.rs"]
mod p1;
#[cfg(test)]
mod tests {
    use crate::p1::Solution;

    #[test]
    fn test_case_1() {
        let nums: Vec<i32> = [2, 7, 11, 15].to_vec();
        let target: i32 = 9;
        let expected: Vec<i32> = [0, 1].to_vec();
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
    #[test]
    fn test_case_2() {
        let nums: Vec<i32> = [3, 2, 4].to_vec();
        let target: i32 = 6;
        let expected: Vec<i32> = [1, 2].to_vec();
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
    #[test]
    fn test_case_3() {
        let nums: Vec<i32> = [3, 3].to_vec();
        let target: i32 = 6;
        let expected: Vec<i32> = [0, 1].to_vec();
        assert_eq!(Solution::two_sum(nums, target), expected);
    }
}
