/*
https://leetcode.com/contest/weekly-contest-373/problems/count-beautiful-substrings-i/
*/

impl Solution {
    pub fn is_vowel(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            _ => false
        }
    }
    pub fn beautiful_substrings(s: String, k: i32) -> i32 {
        let n: usize = s.len();
        let mut cnt_vowels = vec![0; n + 1];
        let mut cnt_consonants = vec![0; n + 1];
        
        let mut idx: usize = 1;
        for ch in s.chars() {
            cnt_vowels[idx] = cnt_vowels[idx - 1];
            cnt_consonants[idx] = cnt_consonants[idx - 1];
            if Self::is_vowel(ch) {
                cnt_vowels[idx] += 1;
            } else {
                cnt_consonants[idx] += 1;
            }
            idx += 1;
        }
        //println!("{:?}", cnt_vowels);
        //println!("{:?}", cnt_consonants);
        
        let mut res = 0;
        for i in 1..n {
            for j in (i + 1)..=n {
                let vowel = cnt_vowels[j] - cnt_vowels[i - 1];
                let consonant = cnt_consonants[j] - cnt_consonants[i - 1];
                if (vowel == consonant) && ((vowel * consonant) % k == 0) {
                    res += 1;
                }
            }
        }
        return res;
    }
}
