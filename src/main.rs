mod p1;
use p1::Solution;
fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    let s = Solution::two_sum(nums, target);
    println!("{:?}", s);
}