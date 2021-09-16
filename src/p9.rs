pub struct Solution {
}
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        };
        let mut res: i64 = 0;
        let mut temp: i64 = x as i64;
        while temp != 0 {
            res = res * 10 + temp % 10;
            temp = temp / 10;
        }
        return res == x as i64;
    }
}