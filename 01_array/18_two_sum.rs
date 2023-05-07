/*
https://leetcode.com/problems/two-sum/description/
*/


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut used: HashMap<i32, usize> = HashMap::new(); // value -> index in vector

        for i in 0..nums.len() {
            if used.contains_key(&(target - nums[i])) {
                let index = used[&(target - nums[i])];
                return vec![index as i32, i as i32];
            } else {
                used.insert(nums[i], i);
            }
        }
        return vec![-1, -1]; // error code
    }
}
