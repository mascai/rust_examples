/*
https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/description/
*/

use std::collections::HashSet;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
        h_fences.push(1);
        h_fences.push(m);
        h_fences.sort();

        v_fences.push(1);
        v_fences.push(n);
        v_fences.sort();

        let mut cnt: HashSet<i32> = HashSet::new();
        for i in 0..h_fences.len() {
            for j in (i + 1)..h_fences.len() {
                cnt.insert(h_fences[j] - h_fences[i]);
            }
        }
        let mut res: i64 = -1;
        for i in 0..v_fences.len() {
            for j in (i + 1)..v_fences.len() {
                let diff = (v_fences[j] - v_fences[i]);
                if cnt.contains(&diff) {
                    res = res.max(diff as i64);
                }
            }
        }
        if res == -1 {
            return -1;
        }
        let MOD: i64 = 10_i64.pow(9) + 7; 
        return ((res * res) % MOD) as i32;
    }
}
