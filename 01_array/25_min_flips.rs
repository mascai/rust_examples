/*
https://leetcode.com/submissions/detail/965845514/
*/

impl Solution {
    pub fn min_flips(mut a: i32, mut b: i32, mut c: i32) -> i32 {
        let mut res = 0;
        while a > 0 || b > 0 || c > 0 {
            let mut bita = a % 2; 
            let mut bitb = b % 2; 
            let mut bitc = c % 2;

            a /= 2;
            b /= 2;
            c /= 2; 

            match bita + bitb {
                0 => {
                    if bitc == 1 {
                        res += 1;
                    }
                }
                1 => {
                    if bitc == 0 {
                        res += 1;
                    }
                }
                2 => { // 2
                    if bitc == 0 {
                        res += 2;
                    }
                }
                _ => panic!("Invalid value")
            }
        }
        return res;
    }
}
