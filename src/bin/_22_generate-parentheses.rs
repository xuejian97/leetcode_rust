/*
Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.



Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
Example 2:

Input: n = 1
Output: ["()"]


Constraints:

1 <= n <= 8
 */
use std::collections::VecDeque;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut queue = VecDeque::new();
        queue.push_back(("(".to_owned(), n - 1, n));
        for i in 1..n * 2 {
            for _ in 0..queue.len() {
                let x = queue.pop_front().unwrap();
                if x.1 > 0 {
                    queue.push_back((format!("{}{}", x.0, "("), x.1 - 1, x.2));
                }
                if x.2 > x.1 {
                    queue.push_back((format!("{}{}", x.0, ")"), x.1, x.2 - 1));
                }
            }
        }
        queue.iter().map(|(x, _, _)| x.clone()).collect()
    }
}

struct Solution;

fn main() {}