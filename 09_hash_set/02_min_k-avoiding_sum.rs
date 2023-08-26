/*
2829. Determine the Minimum Sum of a k-avoiding Array
User Accepted:13015
User Tried:14681
Total Accepted:13459
Total Submissions:25041
Difficulty:Medium
You are given two integers, n and k.

An array of distinct positive integers is called a k-avoiding array if there does not exist any pair of distinct elements that sum to k.

Return the minimum possible sum of a k-avoiding array of length n.

 

Example 1:

Input: n = 5, k = 4
Output: 18
Explanation: Consider the k-avoiding array [1,2,4,5,6], which has a sum of 18.
It can be proven that there is no k-avoiding array with a sum less than 18.
Example 2:

Input: n = 2, k = 6
Output: 3
Explanation: We can construct the array [1,2], which has a sum of 3.
It can be proven that there is no k-avoiding array with a sum less than 3.
 

Constraints:

1 <= n, k <= 50
*/


use std::collections::HashSet;

impl Solution {
    pub fn minimum_sum(n: i32, k: i32) -> i32 {
        let mut cnt: HashSet<i32> = HashSet::new();
        let mut sum: i32 = 0;
        let mut cur = 1;
        while cnt.len() != n as usize {
            if !cnt.contains(&(k - cur)) {
                cnt.insert(cur);
                sum += cur;
            }
            cur += 1;
        }
        return sum;
    }
}
