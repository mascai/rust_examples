/*
https://leetcode.com/problems/duplicate-zeros/description/

*/


impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut i: usize = 0;
        let sz = arr.len();
        while i < sz {
            if arr[i] == 0 {
                arr.insert(i, 0);
                i += 2;
            } else {
                i += 1;
            }
        }
        arr.resize(sz, 0);
    }
}
