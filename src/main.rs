mod p7;
use p7::Solution;
fn main() {
    let x = 1534236469;
    let s = Solution::reverse(x);
    println!("{:?}", s);
}