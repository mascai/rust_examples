/*
https://leetcode.com/problems/fizz-buzz/

Given an integer n, return a string array answer (1-indexed) where:

answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
answer[i] == "Fizz" if i is divisible by 3.
answer[i] == "Buzz" if i is divisible by 5.
answer[i] == i (as a string) if none of the above conditions are true.

*/


// Solution1

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {

        let res = (1..(n+1)).map(|i| {
            if i % 15 == 0 {
                "FizzBuzz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else {
                i.to_string()
            }
        }).collect();
        return res;
    }
}


// Solution2

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::new();

        for i in 1..(n+1) {
            if i % 15 == 0 {
                res.push("FizzBuzz".to_string());
            } else if i % 5 == 0 {
                res.push("Buzz".to_string());
            } else if i % 3 == 0 {
                res.push("Fizz".to_string());
            } else {
                res.push(i.to_string());
            }
        }
        return res;
    }
}
