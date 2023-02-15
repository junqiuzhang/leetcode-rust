use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut num = 0;
        let roman_num_arr = ['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        let dec_num_arr = [1, 5, 10, 50, 100, 500, 1000];
        let mut num_map = HashMap::new();
        for i in 0..7 {
            num_map.insert(roman_num_arr[i], dec_num_arr[i]);
        }
        let mut i = (s.len() - 1) as i32;
        let mut pre = 0;
        let mut cur;
        while i >= 0 {
            cur = num_map[&s.chars().nth(i as usize).unwrap()];
            if cur >= pre {
                num += cur;
            } else {
                num -= cur;
            }
            pre = cur;
            i -= 1;
        }
        return num;
    }
}
