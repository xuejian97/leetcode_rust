/*
Given a string containing just the characters '(' and ')', return the length of the longest valid (well-formed) parentheses
substring
.



Example 1:

Input: s = "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()".
Example 2:

Input: s = ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()".
Example 3:

Input: s = ""
Output: 0


Constraints:

0 <= s.length <= 3 * 104
s[i] is '(', or ')'.
 */
use std::cmp::max;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<(u8, i32)> = vec![];
        for (i, &x) in s.as_bytes().iter().enumerate() {
            if x == 40 {
                stack.push((x, i as i32));
                continue;
            }
            if stack.len() > 0 && stack.last().unwrap().0 == 40 {
                stack.pop();
            } else {
                stack.push((x, i as i32));
            }
        };
        if stack.len() == 0 { return s.len() as i32; }
        if stack[0].1 != 0 {
            stack.insert(0, (0, -1));
        }
        if stack.last().unwrap().1 != s.len() as i32 - 1 {
            stack.push((0, s.len() as i32));
        }
        let mut ans = 0;
        for i in 1..stack.len() {
            ans = max(stack[i].1 - stack[i - 1].1 - 1, ans);
        }
        ans as i32
    }
}

struct Solution;

fn main() {
    println!("{},{}", '(' as u8, ')' as u8)
}