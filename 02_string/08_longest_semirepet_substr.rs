/*
https://leetcode.com/problems/find-the-longest-semi-repetitive-substring/

You are given a 0-indexed string s that consists of digits from 0 to 9.

A string t is called a semi-repetitive if there is at most one consecutive pair of the same digits inside t.

Return the length of the longest semi-repetitive substring inside s.

A substring is a contiguous non-empty sequence of characters within a string.

 

Example 1:

Input: s = "52233"
Output: 4
Explanation: The longest semi-repetitive substring is "5223", which starts at i = 0 and ends at j = 3. 
Example 2:

Input: s = "5494"
Output: 4
Explanation: s is a semi-reptitive string, so the answer is 4.
Example 3:

Input: s = "1111111"
Output: 2
Explanation: The longest semi-repetitive substring is "11", which starts at i = 0 and ends at j = 1.
 

Constraints:

1 <= s.length <= 50
'0' <= s[i] <= '9'
*/


impl Solution {
    pub fn check(s: &str) -> bool {
        let mut cnt_dups = 0;
        let mut prev = s.chars().nth(0).unwrap();
        for ch in s[1..].chars() {
            if ch == prev {
                cnt_dups += 1;
            }
            prev = ch;
        }
        let res = cnt_dups <= 1;
        // println!("{s} -- {cnt_dups} {res}");
        return res;
    }

    pub fn longest_semi_repetitive_substring(s: String) -> i32 {
        let mut res = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                if Self::check(&s[i..j+1]) {
                    res = std::cmp::max(res, (j - i + 1) as i32);
                }
            }
        }
        return res;
    }
}
