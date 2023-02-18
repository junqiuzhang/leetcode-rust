pub struct Solution {}
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }
        let first_str = &strs[0];
        let mut prefix = String::new();
        for (i, c) in first_str.chars().enumerate() {
            for s in strs.iter().skip(1) {
                if i >= s.len() || s.chars().nth(i) != Some(c) {
                    return prefix;
                }
            }
            prefix.push(c);
        }
        return prefix;
    }
}
