use std::collections::HashMap;
pub struct Solution {}
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            nums_map.insert(value, index as i32);
        }
        for (i, v) in nums.iter().enumerate() {
            let value1 = v;
            let value2 = target - value1;
            let index1 = i as i32;
            if nums_map.contains_key(&value2) {
                let index2 = nums_map[&value2];
                if index1 != index2 {
                    return vec![index1, index2];
                }
            }
        }
        return vec![];
    }
}
