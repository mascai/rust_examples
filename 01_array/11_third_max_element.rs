/*
https://leetcode.com/problems/third-maximum-number/editorial/
*/


impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max1: i32 = i32::MIN;
        let mut max2: i32 = i32::MIN;
        let mut max3: i32 = i32::MIN;
        
        for i in 0..nums.len() {
            if nums[i] == max1 || nums[i] == max2 || nums[i] == max3 {
                continue;
            }
            if nums[i] > max1 {
                max3 = max2;
                max2 = max1;
                max1 = nums[i];
            } else if nums[i] > max2 {
                max3 = max2;
                max2 = nums[i];
            } else if nums[i] > max3 {
                max3 = nums[i];
            }
        }
        if max3 == i32::MIN {
            return max1;
        }
        return max3;
    }
}
