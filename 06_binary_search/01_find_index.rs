/*
https://leetcode.com/problems/binary-search/description/?envType=study-plan&id=algorithm-i
*/

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = nums.len() as i32 - 1;
        while low <= high {
            let mut mid = (low + high) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[mid as usize] > target {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return -1;
    }
}
