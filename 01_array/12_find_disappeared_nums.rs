/*
https://leetcode.com/explore/learn/card/fun-with-arrays/523/conclusion/3270/

*/

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut cnt = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            cnt[nums[i] as usize] = 1;
        }
        
        let mut res: Vec<i32> = Vec::new();
        for i in 1..cnt.len() {
            if cnt[i] == 0 {
                res.push(i as i32);
            }
        }
        return res;
    }
}
