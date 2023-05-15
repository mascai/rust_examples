/*
https://leetcode.com/explore/interview/card/top-interview-questions-easy/127/strings/887/

*/

// Solution1
impl Solution {
    pub fn longest_pref<'a>(mut s1: &'a str, mut s2: &'a str) -> &'a str {
        let mut idx: usize = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 != c2 {
                break;
            }
            idx += c1.len_utf8();
        }
        return &s1[0..idx];
    }

    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut pref: &str = &strs[0];
        for i in 1..strs.len() {
            pref = Self::longest_pref(&pref, &strs[i]);
        }
        return pref.to_string();
    }
}


// Solution2

impl Solution {
    pub fn get_common_prefix<'a>(mut s1: &'a str, mut s2: &'a str) -> &'a str { 
        let mut idx: usize = 0;
        if s1.len() > s2.len() {
            std::mem::swap(&mut s1, &mut s2);
        }
        
        println!("{} {}", s1, s2);
        while idx < s1.len() && s1.chars().nth(idx) == s2.chars().nth(idx) {
            idx += 1;    
        }
        return &s1[0..idx];
        
        
    }
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        strs.sort();
        let mut longest_pref: &str = &strs[0];
        for i in 1..strs.len() {
            longest_pref = Self::get_common_prefix(&longest_pref, &strs[i]);
        }
        return longest_pref.to_string();
    }
}
