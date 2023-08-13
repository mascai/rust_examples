/*
https://leetcode.com/contest/weekly-contest-358/problems/max-pair-sum-in-an-array/
*/

use std::collections::HashMap;


impl Solution {
    pub fn max_num(mut n: i32) -> i32{
        let mut max_num = n % 10;
        n /= 10;
        while n > 0 {
            let cur_num = n % 10;
            max_num = std::cmp::max(max_num, cur_num);
            n /= 10;
        }
        return max_num;
    }

    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max_nums = HashMap::new();
        for i in 0..nums.len() {
            if !max_nums.contains_key(&nums[i]) {
                max_nums.insert(nums[i], Self::max_num(nums[i]));
            }
        }
        
        let mut res = -1;
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if max_nums.get(&nums[i]) == max_nums.get(&nums[j]) {
                    res = std::cmp::max(res, nums[i] + nums[j]);
                }
            }
        }
        return res;
    }
}
