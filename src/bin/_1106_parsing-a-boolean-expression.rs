/*
A boolean expression is an expression that evaluates to either true or false. It can be in one of the following shapes:

't' that evaluates to true.
'f' that evaluates to false.
'!(subExpr)' that evaluates to the logical NOT of the inner expression subExpr.
'&(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical AND of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
'|(subExpr1, subExpr2, ..., subExprn)' that evaluates to the logical OR of the inner expressions subExpr1, subExpr2, ..., subExprn where n >= 1.
Given a string expression that represents a boolean expression, return the evaluation of that expression.

It is guaranteed that the given expression is valid and follows the given rules.



Example 1:

Input: expression = "&(|(f))"
Output: false
Explanation:
First, evaluate |(f) --> f. The expression is now "&(f)".
Then, evaluate &(f) --> f. The expression is now "f".
Finally, return false.
Example 2:

Input: expression = "|(f,f,f,t)"
Output: true
Explanation: The evaluation of (false OR false OR false OR true) is true.
Example 3:

Input: expression = "!(&(f,t))"
Output: true
Explanation:
First, evaluate &(f,t) --> (false AND true) --> false --> f. The expression is now "!(f)".
Then, evaluate !(f) --> NOT false --> true. We return true.


Constraints:

1 <= expression.length <= 2 * 104
expression[i] is one following characters: '(', ')', '&', '|', '!', 't', 'f', and ','.
 */

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        use std::collections::VecDeque;

        let expression_bytes = expression.as_bytes();
        let mut stack = VecDeque::new();
        let mut index = 0;
        while index < expression.len() {
            let char = expression_bytes[index] as char;
            match char {
                ')' => {
                    let mut args = vec![];
                    while !vec!['&', '|', '!'].contains(stack.back().unwrap()) {
                        args.push(stack.pop_back().unwrap());
                    }
                    match stack.pop_back().unwrap() {
                        '&' => {
                            stack.push_back(Solution::and(args));
                        }
                        '|' => {
                            stack.push_back(Solution::or(args));
                        }
                        '!' => {
                            stack.push_back(Solution::not(args[0]));
                        }
                        _ => {}
                    }
                }
                '&' | '|' | '!' => {
                    stack.push_back(char);
                    index += 1;
                }
                't' | 'f' => {
                    stack.push_back(char);
                }
                _ => {}
            }
            index += 1;
        }
        if stack.back().unwrap() == &'t' { true } else { false }
    }

    pub fn and(args: Vec<char>) -> char {
        if args.contains(&'f') { 'f' } else { 't' }
    }

    pub fn or(args: Vec<char>) -> char {
        if args.contains(&'t') { 't' } else { 'f' }
    }

    pub fn not(arg: char) -> char {
        if arg == 't' { 'f' } else { 't' }
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::parse_bool_expr("&(|(f))".to_owned()),
        false
    );
}