/*
https://leetcode.com/problems/spiral-matrix/submissions/947072575/

*/


impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut left = 0 as usize;
        let mut right = m - 1 as usize;
        let mut top = 0 as usize;
        let mut bot = n - 1 as usize;

        let mut mode = 0;
        let mut idx = 0 as usize;
        let mut res: Vec<i32> = Vec::new();
        while idx <= n * m - 1 {
            println!("{}:  {}", mode%4, idx);
            if mode % 4 == 0 {
                for i in left..=right {
                    res.push(matrix[top][i]);
                    idx += 1;
                } 
                mode += 1;
                top += 1;
                continue;
            } else if mode % 4 == 1 {
                for i in top..=bot {
                    res.push(matrix[i][right]);
                    idx += 1;
                }
                mode += 1;
                right -= 1;
                continue;
            } else if mode % 4 == 2 {
                for i in (left..=right).rev() {
                    res.push(matrix[bot][i]);
                    idx += 1;
                }
                mode += 1;
                bot -= 1;
                continue;
            } else if mode % 4 == 3 {
                for i in (top..=bot).rev() {
                    res.push(matrix[i][left]);
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
