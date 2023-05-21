/*
https://leetcode.com/problems/minimum-string-length-after-removing-substrings/solutions/3546742/rust-python-stack-check-if-can-remove-last-otherwise-add/
*/

// Solution 1
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut res = Vec::new();
        for ch in s.chars() {
            if (ch == 'B' && !res.is_empty() && res[res.len() - 1] == 'A') ||
               (ch == 'D' && !res.is_empty() && res[res.len() - 1] == 'C') {
                res.pop();
            } else {
                res.push(ch);
            }
        }
        return res.len() as i32;
    }
}

// Solution 2
impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut res = s.clone();
        while res.contains("AB") || res.contains("CD") {
            res = res.replace("AB", "");
            res = res.replace("CD", "");
        }
        return res.len() as i32;
    }
}
