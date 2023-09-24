/*
https://leetcode.com/problems/maximum-odd-binary-number/solutions/4084265/rust-beats-100-solutions/
*/

impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut cnt0: usize = 0;
        let mut cnt1: usize = 0;
        for ch in s.chars() {
            if ch == '0' {
                cnt0 += 1;
            } else {
                cnt1 += 1;
            }
        }
        let res = "1".repeat(cnt1 - 1) + &"0".repeat(cnt0) + "1";
        return res;
    }
}
