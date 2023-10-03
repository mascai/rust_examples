/*
https://leetcode.com/problems/number-of-good-pairs/description/?envType=daily-question&envId=2023-10-03
*/


use std::collections::HashMap;


impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for i in 0..nums.len() {
            if cnt.contains_key(&nums[i]) {
                res += cnt[&nums[i]];
            }
            cnt.entry(nums[i]).and_modify(|n| {*n += 1;}).or_insert(1);
        }
        return res;
    }
}
