/*
https://leetcode.com/problems/find-unique-binary-string/description/?envType=daily-question&envId=2023-11-16
*/

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut res = String::new();

        for i in 0..nums.len() {
            if nums[i].as_bytes()[i] == '0' as u8 {
                res.push('1');
            } else {
                res.push('0');
            }
        }
        return res;
    }
}
