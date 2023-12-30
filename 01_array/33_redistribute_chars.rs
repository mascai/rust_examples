/*
https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
*/

impl Solution {
    pub fn count_chars(cnt: &mut Vec<i32>, word: &str) {
        for ch in word.chars() {
            let idx: usize = ch as usize - 'a' as usize;
            cnt[idx] += 1;
        }
    }

    pub fn make_equal(words: Vec<String>) -> bool {
        let mut cnt = vec![0; 26];
        for word in words.iter() {
            Self::count_chars(&mut cnt, word);
        }
        for i in 0..cnt.len() {
            if cnt[i] % (words.len() as i32) != 0 {
                return false;
            } 
        }
        return true;
    }
}
