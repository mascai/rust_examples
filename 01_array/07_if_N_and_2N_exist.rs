/*
Given an array arr of integers, check if there exist two indices i and j such that :

i != j
0 <= i, j < arr.length
arr[i] == 2 * arr[j]
 

Example 1:

Input: arr = [10,2,5,3]
Output: true
Explanation: For i = 0 and j = 2, arr[i] == 10 == 2 * 5 == 2 * arr[j]
Example 2:

Input: arr = [3,1,7,11]
Output: false
Explanation: There is no i and j that satisfy the conditions.

*/


use std::collections::HashSet;


impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut nums = HashSet::new();
        
        for i in 0..arr.len() {
            if nums.contains(&(arr[i] * 2)) || (arr[i] % 2 == 0 && nums.contains(&(arr[i] / 2))) {
                return true;
            }
            nums.insert(arr[i]);
        }
        return false;
    }
}
