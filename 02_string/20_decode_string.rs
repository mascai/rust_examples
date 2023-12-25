/*
https://leetcode.com/problems/decode-ways/description/?envType=daily-question&envId=2023-12-25
*/

impl Solution {
    pub fn to_int(ch: char) -> i32 {
        return ch as i32 - '0' as i32;
    }
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![0; n + 1];
        if s.chars().next() == Some('0') {
            return 0;
        }
        dp[0] = 1;
        dp[1] = 1;

        let mut iter = s.chars();
        let mut prev: char = iter.next().unwrap();
        let mut idx: usize = 2;
        while let Some(ch) = iter.next() {
            let num1 = Self::to_int(ch);
            let num2 = num1 + Self::to_int(prev) * 10;
            prev = ch;

            if num1 != 0 {
                dp[idx] += dp[idx - 1];
            }
            if num2 > 9 && num2 <= 26 {
                dp[idx] += dp[idx - 2]; 
            }
            idx += 1;
        }
        return dp[n];
    }
}
