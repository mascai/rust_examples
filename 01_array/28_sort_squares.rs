/*
https://leetcode.com/problems/squares-of-a-sorted-array/description/
*/

use std::cmp;


impl Solution {
    pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
        if nums[0] >= 0 {
            return nums.iter().map(|x| x * x).collect();
        }
        if nums[nums.len() - 1] < 0 {
            return nums.iter().rev().map(|x| x * x).collect();
        }
        let mut l = 0 as usize;
        let mut r = nums.len() - 1;
        let mut res = vec![];
        while (l <= r) {
            if l == r {
                res.push((nums[l] * nums[l]) as i32);
                break;
            }
            if nums[l].abs() > nums[r].abs() {
                res.push((nums[l] * nums[l]) as i32);
                l += 1;
            } else {
                res.push((nums[r] * nums[r]) as i32);
                r -= 1;
            }
        }
        res.reverse();
        return res;
    }
}
