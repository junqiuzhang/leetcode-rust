#[path = "../problems/p13.rs"]
mod p13;
#[cfg(test)]
mod p13_tests {
    use crate::p13::Solution;

    #[test]
    fn test_case_1() {
        let s = String::from("III");
        let expected = 3;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
    #[test]
    fn test_case_2() {
        let s = String::from("IV");
        let expected = 4;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
    #[test]
    fn test_case_3() {
        let s = String::from("IX");
        let expected = 9;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
    #[test]
    fn test_case_4() {
        let s = String::from("LVIII");
        let expected = 58;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
    #[test]
    fn test_case_5() {
        let s = String::from("MCMXCIV");
        let expected = 1994;
        assert_eq!(Solution::roman_to_int(s), expected);
    }
}
