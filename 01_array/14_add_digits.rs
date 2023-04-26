/* 
https://leetcode.com/problems/add-digits/description/

Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

 

Example 1:

Input: num = 38
Output: 2
Explanation: The process is
38 --> 3 + 8 --> 11
11 --> 1 + 1 --> 2 
Since 2 has only one digit, return it.
Example 2:

Input: num = 0
Output: 0
*/


impl Solution {
    pub fn add_digits(num: i32) -> i32 {
        let mut res = num;
        while res > 9 {
            let mut cur_sum = 0;
            while res > 0 {
                cur_sum += res % 10;
                res /= 10;
            }
            res = cur_sum;
        }
        return res
    }
}
