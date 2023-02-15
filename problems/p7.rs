pub struct Solution {}
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut res: i32 = 0;
        let mut temp = x;
        while temp != 0 {
            let (_, boo) = res.overflowing_mul(10);
            if boo {
                return 0;
            }
            let (_, boo) = (res * 10).overflowing_add(temp % 10);
            if boo {
                return 0;
            }
            res = res * 10 + temp % 10;
            temp = temp / 10;
        }
        return res;
    }
}
