/* https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3238/

Given a binary array nums, return the maximum number of consecutive 1's in the array.

 

Example 1:

Input: nums = [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.
Example 2:

Input: nums = [1,0,1,1,0,1]
Output: 2
*/

use std::cmp;


impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut cur: i32 = 0;
        for cur_elem in &nums {
            if *cur_elem == 1 {
                cur = cur + 1;
                res = cmp::max(res, cur);
            } else {
                cur = 0;
            }
        }
        return res;
    }
}
