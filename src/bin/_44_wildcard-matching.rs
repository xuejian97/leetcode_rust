/*
Given an input string (s) and a pattern (p), implement wildcard pattern matching with support for '?' and '*' where:

'?' Matches any single character.
'*' Matches any sequence of characters (including the empty sequence).
The matching should cover the entire input string (not partial).



Example 1:

Input: s = "aa", p = "a"
Output: false
Explanation: "a" does not match the entire string "aa".
Example 2:

Input: s = "aa", p = "*"
Output: true
Explanation: '*' matches any sequence.
Example 3:

Input: s = "cb", p = "?a"
Output: false
Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.


Constraints:

0 <= s.length, p.length <= 2000
s contains only lowercase English letters.
p contains only lowercase English letters, '?' or '*'.
 */

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p == "*".to_owned() || (p.len() == 0 && s.len() == 0) { return true; }
        let p_bytes = p.as_bytes();
        let s_bytes = s.as_bytes();
        if s.len() == 0 && p.len() != 0 {
            for x in p_bytes {
                if *x != 42 { return false; }
            }
            return true;
        }
        if s.len() == 0
            || p.len() == 0
            || (*p_bytes.last().unwrap() != 42 && *p_bytes.last().unwrap() != 63 && s_bytes.last().unwrap() != p_bytes.last().unwrap()) {
            return false;
        }
        let mut dp = vec![vec![false; s.len()]; p.len()];
        for i in 0..p.len() {
            for j in 0..s.len() {
                if i == 0 {
                    if p_bytes[0] == 42 {
                        dp[i][j] = true;
                    } else if j == 0 {
                        if p_bytes[0] == 63 || s_bytes[0] == p_bytes[0] {
                            dp[i][j] = true;
                        }
                    }
                    continue;
                }
                if j == 0 {
                    if p_bytes[i] == 42 {
                        dp[i][j] = dp[i - 1][j];
                    } else {
                        let mut p_left = vec![];
                        for k in 0..=i {
                            if p_bytes[k] != 42 {
                                p_left.push(p_bytes[k]);
                            }
                        }
                        dp[i][j] = p_left.len() == 1 && (p_left[0] == 63 || p_left[0] == s_bytes[0]);
                    }
                    continue;
                }

                if p_bytes[i] == 42 {
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                } else if p_bytes[i] == 63 {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j - 1] && p_bytes[i] == s_bytes[j];
                }
            }
        }

        dp[p.len() - 1][s.len() - 1]
    }
}

struct Solution;

fn main() {
    // println!("a: {},z: {},*: {},?: {}", 'a' as u8, 'z' as u8, '*' as u8, '?' as u8);
    // Solution::is_match("acdcb".to_owned(), "a*c?b".to_owned());
    // Solution::is_match("aa".to_owned(), "a*".to_owned());
    // Solution::is_match("ab".to_owned(), "?*".to_owned());
    Solution::is_match("abcabczzzde".to_owned(), "*abc???de*".to_owned());
}