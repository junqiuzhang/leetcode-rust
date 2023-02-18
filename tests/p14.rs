#[path = "../problems/p14.rs"]
mod p14;
#[cfg(test)]
mod p13_tests {
    use crate::p14::Solution;

    #[test]
    fn test_case_1() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        let expected = "fl";
        assert_eq!(Solution::longest_common_prefix(strs), expected);
    }
    #[test]
    fn test_case_2() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        let expected = "";
        assert_eq!(Solution::longest_common_prefix(strs), expected);
    }
}
