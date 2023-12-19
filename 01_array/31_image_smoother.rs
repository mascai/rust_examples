/*
https://leetcode.com/problems/image-smoother/submissions/?envType=daily-question&envId=2023-12-19
*/

impl Solution {
    pub fn calc(i: i32, j: i32, img: &Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        for ii in (i - 1)..=(i + 1) {
            for jj in (j - 1)..=(j + 1) {
                if ii >= 0 && (ii as usize) < img.len() && jj >= 0 && (jj as usize) < img[0].len() {
                    res += img[ii as usize][jj as usize];
                    cnt += 1;
                }
            }
        }
        return res / cnt;
    }
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = img.len();
        let m = img[0].len();
        let mut res = vec![vec![0; m]; n];

        for i in 0..n {
            for j in 0..m {
                res[i][j] = Self::calc(i as i32, j as i32, &img);
            }
        }
        return res;
    }
}
