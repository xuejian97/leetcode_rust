/*
Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).



Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
Example 2:

Input: s = "aa", p = "a*"
Output: true
Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".
Example 3:

Input: s = "ab", p = ".*"
Output: true
Explanation: ".*" means "zero or more (*) of any character (.)".


Constraints:

1 <= s.length <= 20
1 <= p.length <= 30
s contains only lowercase English letters.
p contains only lowercase English letters, '.', and '*'.
It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
 */
use std::ops::Index;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp = vec![vec![false; s.len()]; p.len()];

        for i in 0..p.len() {
            for j in 0..s.len() {
                if i == 0 && j == 0 && (p.index(i..=i) == s.index(j..=j) || p.index(0..=0) == ".") {
                    dp[i][j] = true;
                    continue;
                }
                if i > 0 && j == 0 {
                    let aim = s.as_bytes()[0];
                    let mut has_aim = false;
                    let mut stack = vec![];
                    for k in 0..=i {
                        if p.as_bytes()[k] == aim ||p.as_bytes()[k] == 46 {
                            has_aim = true;
                        }
                        if p.as_bytes()[k] == 42 {
                            stack.pop();
                            continue;
                        } else {
                            stack.push(p.as_bytes()[k]);
                        }
                    }
                    dp[i][j] = has_aim && (stack.len() == 0 || stack.len() == 1 && (stack[0] == aim || stack[0] == 46));
                    continue;
                }

                if i == 0 || j == 0 {
                    continue;
                }

                if p.as_bytes()[i] == 46 {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if p.as_bytes()[i] >= 97 && p.as_bytes()[i] <= 122 {
                    dp[i][j] = dp[i - 1][j - 1] && s.as_bytes()[j] == p.as_bytes()[i];
                } else if p.as_bytes()[i] == 42 {
                    if p.as_bytes()[i - 1] == 46 {
                        dp[i][j] = dp[i][j - 1] || (i >= 2 && (dp[i - 2][j - 1] || dp[i - 2][j]));
                    } else {
                        dp[i][j] = dp[i - 1][j] || (i >= 2 && dp[i - 2][j]) || (dp[i][j - 1] && s.as_bytes()[j] == p.as_bytes()[i - 1]);
                    }
                }
            }
        }

        dp[p.len() - 1][s.len() - 1]
    }
}

struct Solution;

fn main() {
    // Solution::is_match("abcccde".to_owned(), "a.*c*de".to_owned());
    // Solution::is_match("aaa".to_owned(), "ab*ac*a".to_owned());
    // Solution::is_match("aab".to_owned(), "b.*".to_owned());
    Solution::is_match("aab".to_owned(), "c*a*b".to_owned());
    // abc  abc.*
    // Solution::is_match("aasdfasdfasdfasdfas".to_owned(), "aasdf.*asdf.*asdf.*asdf.*s".to_owned());
    /*

     */
}