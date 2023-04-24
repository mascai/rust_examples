/*
https://leetcode.com/problems/sort-array-by-parity/description/

Given an integer array nums, move all the even integers at the beginning of the array followed by all the odd integers.

Return any array that satisfies this condition.
*/

impl Solution {
    pub fn sort_array_by_parity(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|a, b| {
           (a % 2).cmp(&(b % 2)) 
        });
        return nums;
    }
}
