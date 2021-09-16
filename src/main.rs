mod p9;
use p9::Solution;
fn main() {
    let x = 121;
    let s = Solution::is_palindrome(x);
    println!("{:?}", s);
}