/*
https://leetcode.com/problems/isomorphic-strings/description/?envType=study-plan-v2&envId=top-interview-150
*/


use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut iter1 = s.chars();
        let mut iter2 = t.chars();
        let mut cnt: HashMap<char, char> = HashMap::new();
        let mut used: HashSet<char> = HashSet::new();

        while let Some(ch1) = iter1.next() {
            let ch2 = iter2.next().unwrap();
            if cnt.contains_key(&ch1) {
                if cnt[&ch1] != ch2 {
                    return false;
                }
            } else if !used.contains(&ch2) {
                cnt.insert(ch1, ch2);
                used.insert(ch2);
            } else {
                return false;
            }
        }
        return true;
    }
}
