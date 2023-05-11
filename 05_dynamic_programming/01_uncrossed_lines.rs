/*
https://leetcode.com/problems/uncrossed-lines/description/
*/


impl Solution {
    pub fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut dp = vec![vec![0; len1 + 1]; len2 + 1];
        for i in 1..=len2 {
            for j in 1..=len1 {
                if nums1[j - 1] == nums2[i - 1] {
                    dp[i][j] = 1 + dp[i - 1][j - 1];
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        return dp[len2][len1];
    }
}
