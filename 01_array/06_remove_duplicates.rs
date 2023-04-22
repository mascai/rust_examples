/* https://leetcode.com/explore/learn/card/fun-with-arrays/526/deleting-items-from-an-array/3248/
Remove duplicates from sorted array

*/


impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut insert_idx: usize = 1;
        let mut cur: i32 = nums[0];
        
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[insert_idx] = nums[i];
                insert_idx += 1;
            }
        }
        return insert_idx as i32;
    }
}
