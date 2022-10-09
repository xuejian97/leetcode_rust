/*
Given a balanced parentheses string s, return the score of the string.

The score of a balanced parentheses string is based on the following rule:

"()" has score 1.
AB has score A + B, where A and B are balanced parentheses strings.
(A) has score 2 * A, where A is a balanced parentheses string.
 

Example 1:

Input: s = "()"
Output: 1
Example 2:

Input: s = "(())"
Output: 2
Example 3:

Input: s = "()()"
Output: 2
 

Constraints:

2 <= s.length <= 50
s consists of only '(' and ')'.
s is a balanced parentheses string.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/score-of-parentheses
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        use std::collections::VecDeque;
        let bytes = s.as_bytes();
        let mut stack = VecDeque::new();
        let lv = -1;
        for &x in bytes {
            if x == '(' as u8 {
                stack.push_back(lv);
            } else {
                let pre = stack.pop_back().unwrap();
                if pre == lv {
                    stack.push_back(1);
                } else {
                    stack.pop_back();
                    stack.push_back(pre * 2);
                }
                if stack.len() > 1 {
                    let mut value = 0;
                    while let Some(v) = stack.back() {
                        if v != &lv {
                            value += stack.pop_back().unwrap();
                        } else {
                            break;
                        }
                    }
                    stack.push_back(value);
                }
            }
        }
        stack.pop_back().unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::score_of_parentheses("()".to_owned()),
        1
    );
    assert_eq!(
        Solution::score_of_parentheses("(())".to_owned()),
        2
    );
    assert_eq!(
        Solution::score_of_parentheses("()()".to_owned()),
        2
    );
    assert_eq!(
        Solution::score_of_parentheses("(()(()))".to_owned()),
        6
    );
}