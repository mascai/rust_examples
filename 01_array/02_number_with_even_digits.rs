/* https://leetcode.com/problems/find-numbers-with-even-number-of-digits/description/
Given an array nums of integers, return how many of them contain an even number of digits.

 

Example 1:

Input: nums = [12,345,2,6,7896]
Output: 2

*/

// Solution 1
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().map(|x| {
            (*x as f64).log10() as i32 + 1
        }).filter(|n| n % 2 == 0)
        .count() as i32
    }
}
      
// Solution 2
impl Solution {
    pub fn check_num(mut num: i32) -> bool {
        let mut counter = 0;
        if num == 0 {
            return false;
        }
        num = num.abs();
        while num > 0 {
            num /= 10;
            counter += 1;
        }
        if counter % 2 == 0 {
            return true;
        }
        return false;
    }

    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while i < nums.len() {
            if Self::check_num(nums[i]) == true {
                res += 1;
            }
            i += 1;
        }
        return res;
    }
}
