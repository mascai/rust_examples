/*
https://leetcode.com/problems/valid-anagram/submissions/
*/


use std::collections::HashMap;


impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut table = HashMap::<char, i32>::new();
        proc_string(&mut table, s, 1);        
        proc_string(&mut table, t, -1);

        return table.values().all(|v| *v == 0);        
    }
}

fn proc_string(table: &mut  HashMap<char, i32>, st: String, inc: i32) {
    for c in st.chars() {
        table
            .entry(c)
            .and_modify(|counter| *counter += inc)
            .or_insert(inc);
    }
}
