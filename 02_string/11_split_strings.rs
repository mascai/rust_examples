/*
https://leetcode.com/contest/weekly-contest-355/problems/split-strings-by-separator/


Given an array of strings words and a character separator, split each string in words by separator.

Return an array of strings containing the new strings formed after the splits, excluding empty strings.

Notes

separator is used to determine where the split should occur, but it is not included as part of the resulting strings.
A split may result in more than two strings.
The resulting strings must maintain the same order as they were initially given.
 

Example 1:

Input: words = ["one.two.three","four.five","six"], separator = "."
Output: ["one","two","three","four","five","six"]
Explanation: In this example we split as follows:

"one.two.three" splits into "one", "two", "three"
"four.five" splits into "four", "five"
"six" splits into "six" 

Hence, the resulting array is ["one","two","three","four","five","six"].

*/


impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        for word in words {
            let parts = word.split(separator);
            for part in parts {
                if part.len() > 0 {
                    res.push(part.to_string());
                }
            }
        }
        return res;
    }
}
