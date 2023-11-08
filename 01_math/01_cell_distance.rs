/*
https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/description/?envType=daily-question&envId=2023-11-08
*/

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let diffx = (sx - fx).abs();
        let diffy = (sy - fy).abs();

        if std::cmp::max(diffx, diffy) == 0 && t == 1 {
            return false;
        }
        return t >= std::cmp::max(diffx, diffy);
    }
}
