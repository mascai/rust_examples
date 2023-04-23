/* https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3251/

Given an array of integers arr, return true if and only if it is a valid mountain array.

Recall that arr is a mountain array if and only if:

arr.length >= 3
There exists some i with 0 < i < arr.length - 1 such that:
arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
arr[i] > arr[i + 1] > ... > arr[arr.length - 1]

*/


impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let sz: usize = arr.len();
      
        let mut idx = 1;
        while (idx < sz) && (arr[idx - 1] < arr[idx]) {
            idx += 1;
        }
        if idx == 1 || idx == sz || arr[idx - 1] == arr[idx] {
            return false;
        }
        let mut flag = false;
        while (idx < sz) && (arr[idx - 1] > arr[idx]) {
            idx += 1;
            flag = true;
        }
        return idx == sz && flag == true;
    }
}
