/*

*/


impl Solution {
    pub fn matrix_sum_queries(n: i32, queries: Vec<Vec<i32>>) -> i64 {
        let size = n as usize;
        let (mut cnt_rows, mut cnt_cols) = (0, 0);
        let mut used_rows = vec![0; size];
        let mut used_cols = vec![0; size];
        let mut res: i64 = 0;
        
        for i in (0..queries.len()).rev() {
            if queries[i][0] == 0 && used_rows[queries[i][1] as usize] == 0 { // rows
                used_rows[queries[i][1] as usize] = 1;
                cnt_rows += 1;
                res += ((n - cnt_cols) * queries[i][2]) as i64;
            } else if queries[i][0] == 1 && used_cols[queries[i][1] as usize] == 0 { // cols
                used_cols[queries[i][1] as usize] = 1;
                cnt_cols += 1;
                res += ((n - cnt_rows) * queries[i][2]) as i64;
            }
        }
        
        return res;
    }
}
