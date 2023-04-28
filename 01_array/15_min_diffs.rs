/*
https://leetcode.com/explore/learn/card/sorting/695/non-comparison-based-sorts/4489/

Given an array of distinct integers arr, find all pairs of elements with the minimum absolute difference of any two elements.

Return a list of pairs in ascending order(with respect to pairs), each pair [a, b] follows

a, b are from arr
a < b
b - a equals to the minimum absolute difference of any two elements in arr
 

Example 1:

Input: arr = [4,2,1,3]
Output: [[1,2],[2,3],[3,4]]
Explanation: The minimum absolute difference is 1. List all pairs with difference equal to 1 in ascending order.
Example 2:

Input: arr = [1,3,6,10,15]
Output: [[1,3]]
Example 3:

Input: arr = [3,8,-10,23,19,-4,-14,27]
Output: [[-14,-10],[19,23],[23,27]]
 

Constraints:

2 <= arr.length <= 105
-106 <= arr[i] <= 106

*/


impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let mut min_diff = std::i32::MAX;
        let mut min_idx = 0;

        for i in 1..arr.len() {
            if min_diff > arr[i] - arr[i - 1] {
                min_diff = arr[i] - arr[i - 1];
                min_idx = i;
            }
        }
        let mut res: Vec<Vec<i32>> = vec![
            vec![arr[min_idx - 1], arr[min_idx]]
        ];

        min_idx += 1;
        for i in min_idx..arr.len() {
            if arr[i] - arr[i - 1] == min_diff {
                res.push(vec![arr[i - 1], arr[i]]);
            }
        }
        return res;
    }
}
