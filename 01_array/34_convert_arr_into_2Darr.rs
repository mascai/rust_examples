/*
https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/description/
*/


impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freq: Vec<usize> = vec![0; (nums.len() + 1)];
        let mut res = vec![vec![]];
        for i in 0..nums.len() {
            if freq[nums[i] as usize] >= res.len() {
                res.push(vec![]);
            }
            res[freq[nums[i] as usize]].push(nums[i]);
            freq[nums[i] as usize] += 1;
        }
        return res;
    }
}
