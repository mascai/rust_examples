/*  https://leetcode.com/problems/sort-vowels-in-a-string/solutions/3803223/video-sorting-vowels-in-a-python-string-a-leetcode-challenge-solved/

*/

// Solution 1

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels = "aeiouAEIOU";
        let mut sorted_vowels: Vec<char> = s.chars()
            .filter(|&ch| { vowels.contains(ch)}).collect();
        sorted_vowels.sort();

        let mut new_string = String::new();
        let mut idx = 0;
        for c in s.chars() {
            if vowels.contains(c) {
                new_string.push(sorted_vowels[idx]);
                idx += 1;
            } else {
                new_string.push(c);
            }
        }
        return new_string;
    }
    
}
    
//Solution 2
impl Solution {
    pub fn is_vowel(ch: char) -> bool {
        return ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' ||
               ch == 'A' || ch == 'E' || ch == 'I' || ch == 'O' || ch == 'U'; 
    }

    pub fn sort_vowels(s: String) -> String {
        let mut vowels = Vec::<char>::new();
        for ch in s.chars() {
            if Self::is_vowel(ch) {
                vowels.push(ch);
            }
        }
        vowels.sort();
        let mut res: Vec<char> = s.chars().collect();
        let mut idx: usize = 0;
        for i in 0..res.len() {
            if Self::is_vowel(res[i]) {
                res[i] = vowels[idx];
                idx += 1;
            }
        }
        let res: String = res.iter().collect();
        return res;
    }
}
