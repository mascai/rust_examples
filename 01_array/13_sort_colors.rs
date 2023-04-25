/*
https://leetcode.com/explore/learn/card/sorting/694/comparison-based-sorts/4483/


Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.

We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.

You must solve this problem without using the library's sort function.

 

Example 1:

Input: nums = [2,0,2,1,1,0]
Output: [0,0,1,1,2,2]
Example 2:

Input: nums = [2,0,1]
Output: [0,1,2]
*/

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut cnt_colors = vec![0; 3];
        for i in 0..nums.len() {
            cnt_colors[nums[i] as usize] += 1;
        }
        for i in 0..nums.len() {
            if cnt_colors[0] > 0 {
                cnt_colors[0] -= 1;
                nums[i] = 0;
            } else if cnt_colors[1] > 0 {
                cnt_colors[1] -= 1;
                nums[i] = 1;
            } else if cnt_colors[2] > 0 {
                cnt_colors[2] -= 1;
                nums[i] = 2;
            }
        }
    }
}
