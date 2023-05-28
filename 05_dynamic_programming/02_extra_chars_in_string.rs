/*
https://leetcode.com/problems/extra-characters-in-a-string/description/
*/

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n: usize = s.len();
        let mut dp = vec![n as i32; n + 1];
        dp[0] = 0;
        for i in 0..n {
            for word in dictionary.iter() {
                //println!("{} {} {}", i, word, s);
                let end_index: usize = std::cmp::min(i + word.len(), s.len());
                if word == &s[i..end_index] {
                    dp[i + word.len()] = std::cmp::min(dp[i + word.len()], dp[i]);
                }
            }
            dp[i + 1] = std::cmp::min(dp[i + 1], dp[i] + 1);
        }
        return dp[dp.len() - 1];
    }
}
