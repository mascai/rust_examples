/*
https://leetcode.com/problems/valid-parentheses/description/

*/


impl Solution {
    pub fn check(open_bracket: char, close_bracket: char) -> bool {
        return (open_bracket == '(' && close_bracket == ')') ||
               (open_bracket == '{' && close_bracket == '}') ||
               (open_bracket == '[' && close_bracket == ']'); 
    }

    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);
            } else {
                if stack.is_empty() {
                    return false;
                }
                let stack_top: char = stack.pop().unwrap();
                if !Self::check(stack_top, ch) {
                    return false;
                }
            }
        }
        return stack.is_empty();
    }
}
