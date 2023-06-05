/*
https://leetcode.com/problems/check-if-it-is-a-straight-line/description/


*/


impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let (x1, y1) = (coordinates[0][0], coordinates[0][1]);
        let (x2, y2) = (coordinates[1][0], coordinates[1][1]);
        let diffx = x2 - x1;
        let diffy = y2 - y1;
        
        for i in 2..coordinates.len() {
            let (curx, cury) = (coordinates[i][0], coordinates[i][1]);
            if diffx * (cury - y1) != diffy * (curx - x1) {
                return false;
            }
        }
        return true;
    }
}
