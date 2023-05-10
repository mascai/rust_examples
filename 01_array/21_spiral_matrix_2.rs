/*
https://leetcode.com/problems/spiral-matrix-ii/submissions/
*/

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut left = 0 as usize;
        let mut right = (n - 1) as usize;
        let mut top = 0 as usize;
        let mut bot = (n - 1) as usize;

        let mut mode = 0;
        let mut idx = 1;
        let mut res = vec![vec![0; n as usize]; n as usize];
        while idx <= n * n {
            if mode % 4 == 0 {
                for i in left..=right {
                    res[top][i] = idx;
                    idx += 1;
                } 
                mode += 1;
                top += 1;
                continue;
            } else if mode % 4 == 1 {
                for i in top..=bot {
                    res[i][right] = idx;
                    idx += 1;
                }
                mode += 1;
                right -= 1;
                continue;
            } else if mode % 4 == 2 {
                for i in (left..=right).rev() {
                    res[bot][i] = idx;
                    idx += 1;
                }
                mode += 1;
                bot -= 1;
                continue;
            } else if mode % 4 == 3 {
                for i in (top..=bot).rev() {
                    res[i][left] = idx;
                    idx += 1;
                }
                left += 1;
                mode += 1;
                continue;
            }
        }
        return res;
    }
}
