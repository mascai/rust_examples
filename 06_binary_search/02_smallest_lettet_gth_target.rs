/*
https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/
*/


impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        if target >= letters[letters.len() - 1] {
            return letters[0];
        }
        let mut l: i32 = 0;
        let mut h: i32 = (letters.len() - 1) as i32;

        while l <= h {
            let mid: i32 = (l + h) / 2;
            println!("{} {} {}", l, h, mid);
            if letters[mid as usize] > target {
                h = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return letters[l as usize];
    }
}
