/*
https://leetcode.com/problems/length-of-last-word/solutions/1320947/rust-one-liner/?envType=study-plan-v2&envId=top-interview-150
*/

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut last_char = -1;
        let mut first_char = -1;
        let mut idx = (s.len() - 1) as i32;
        for ch in s.chars().rev() {
            if last_char == -1 && ch != ' ' {
                last_char = idx;
                first_char = idx;
            }

            if last_char != -1 && ch == ' ' {
                break;
            } else {
                first_char = idx;
            }
            idx -= 1;
        }
        if last_char == -1 {
            return 0;
        }
        return last_char - first_char + 1;
    }
}
