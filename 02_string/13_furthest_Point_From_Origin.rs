/*
https://leetcode.com/problems/furthest-point-from-origin/
*/

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let [cntl, cntr, cnt_other] = moves
            .chars()
            .fold([0, 0, 0], |mut acc, m| {
                match m {
                    'L' => acc[0] += 1,
                    'R' => acc[1] += 1,
                    '_' => acc[2] += 1,
                    _ => {}
                }
                acc
            });
        return (cntl as i32 - cntr).abs() + cnt_other;
    }
}


impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut cntl = 0;
        let mut cntr = 0;
        for ch in moves.chars() {
            if ch == 'L' {
                cntl += 1;
            } else if ch == 'R' {
                cntr += 1;
            }
        }
        let cnt_other = moves.len() as i32 - cntr - cntl;
        let diff = cntl - cntr;
        return diff.abs() + cnt_other;
    }
}
