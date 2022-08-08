/*
Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

The algorithm for myAtoi(string s) is as follows:

Read in and ignore any leading whitespace.
Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
Return the integer as the final result.
Note:

Only the space character ' ' is considered a whitespace character.
Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
 

Example 1:

Input: s = "42"
Output: 42
Explanation: The underlined characters are what is read in, the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)
           ^
The parsed integer is 42.
Since 42 is in the range [-231, 231 - 1], the final result is 42.
Example 2:

Input: s = "   -42"
Output: -42
Explanation:
Step 1: "   -42" (leading whitespace is read and ignored)
            ^
Step 2: "   -42" ('-' is read, so the result should be negative)
             ^
Step 3: "   -42" ("42" is read in)
               ^
The parsed integer is -42.
Since -42 is in the range [-231, 231 - 1], the final result is -42.
Example 3:

Input: s = "4193 with words"
Output: 4193
Explanation:
Step 1: "4193 with words" (no characters read because there is no leading whitespace)
         ^
Step 2: "4193 with words" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "4193 with words" ("4193" is read in; reading stops because the next character is a non-digit)
             ^
The parsed integer is 4193.
Since 4193 is in the range [-231, 231 - 1], the final result is 4193.
 

Constraints:

0 <= s.length <= 200
s consists of English letters (lower-case and upper-case), digits (0-9), ' ', '+', '-', and '.'.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/string-to-integer-atoi
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


use std::ops::Index;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        use std::ops::Index;

        let mut s = s.trim();
        let mut negative = false;
        let mut res = 0i64;
        if s.len() == 0 { return 0; }
        if s.index(0..1) == "-" {
            negative = true;
            s = &s[1..s.len()];
        } else if s.index(0..1) == "+" {
            s = &s[1..s.len()];
        }


        for (i, x) in s.split("").enumerate() {
            if i == 0 { continue; }
            let number: Result<i64, _> = x.parse();
            match number {
                Ok(n) => res = res * 10 + n,
                Err(_) => break
            }

            if res > i32::MAX as i64 {
                return if negative {
                    i32::MIN
                } else {
                    i32::MAX
                };
            }
        }
        if negative { res *= -1 }
        res as i32
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::my_atoi("42".to_owned()));
    println!("{}", Solution::my_atoi("-42".to_owned()));
    println!("{}", Solution::my_atoi("4193 with words".to_owned()));
    println!("{}", Solution::my_atoi("41 93".to_owned()));
    println!("{}", Solution::my_atoi("".to_owned()));
    println!("{}", Solution::my_atoi("9223372036854775808".to_owned()));
}