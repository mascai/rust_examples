/*
https://leetcode.com/problems/repeated-dna-sequences/description/
*/

use std::collections::HashMap;


impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 11 {
            return Vec::new();
        }
        let mut res = Vec::new();
        let mut used: HashMap<&str, i32> = HashMap::new();

        let mut i: usize = 0;
        while i < s.len() - 9 {
            let cur = &s[i..(i + 10)];
            used.entry(cur).and_modify(|num| {
                *num += 1;
                if *num == 2 {
                    res.push(cur.to_string());
                }
            }).or_insert(1);
            i += 1;
        }
        return res;
    }
}
