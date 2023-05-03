/*
https://leetcode.com/problems/find-the-difference-of-two-arrays/description/
*/


use std::collections::HashSet;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
        let hash1: HashSet<i32> = nums1.iter().cloned().collect();
        let hash2: HashSet<i32> = nums2.iter().cloned().collect();

        let diff1: HashSet<i32> = hash1.difference(&hash2).cloned().collect();
        let diff2: HashSet<i32> = hash2.difference(&hash1).cloned().collect();

        return vec![
            diff1.into_iter().collect(), 
            diff2.into_iter().collect()
        ];
    }
}
