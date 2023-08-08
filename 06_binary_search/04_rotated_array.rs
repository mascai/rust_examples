/*

https://leetcode.com/problems/search-in-rotated-sorted-array/description/
*/


impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0 as usize;
        let mut high = nums.len() - 1;
        while low <= high {
            let mid = (low + high) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] >= nums[0] {
                if nums[0] <= target && target < nums[mid] {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else if nums[mid] < target && target <= nums[nums.len() - 1] {
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }
        return -1;
    }
}
