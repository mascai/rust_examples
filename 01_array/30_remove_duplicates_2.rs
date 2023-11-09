/*
https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/description/?envType=study-plan-v2&envId=top-interview-150
*/


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return nums.len() as i32;
        }
        let mut insert_idx = 2;
        for i in 2..nums.len() {
            if nums[i] != nums[insert_idx - 1] || nums[i] != nums[insert_idx - 2] {
                nums[insert_idx] = nums[i];
                insert_idx += 1;
            }
        }
        return insert_idx as i32;
    }
}
