/*
https://leetcode.com/problems/sign-of-the-product-of-an-array/description/
*/

impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for cur in nums {
            if cur == 0 {
                return 0;
            }
            if cur < 0 {
                sign *= -1;
            }
        }
        return sign;
    }
}
