/*
Special binary strings are binary strings with the following two properties:

The number of 0's is equal to the number of 1's.
Every prefix of the binary string has at least as many 1's as 0's.
You are given a special binary string s.

A move consists of choosing two consecutive, non-empty, special substrings of s, and swapping them. Two strings are consecutive if the last character of the first string is exactly one index before the first character of the second string.

Return the lexicographically largest resulting string possible after applying the mentioned operations on the string.

   

Example 1:

Input: s = "11011000"
Output: "11100100"
Explanation: The strings "10" [occuring at s[1]] and "1100" [at s[3]] are swapped.
This is the lexicographically largest string possible after some number of swaps.
Example 2:

Input: s = "10"
Output: "10"
   

Constraints:

1 <= s.length <= 50
s[i] is either '0' or '1'.
s is a special binary string.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/special-binary-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        use std::ops::Index;

        if s.len() <= 2 {
            return s;
        }

        let (mut cnt, mut left) = (0, 0);
        let mut subs:Vec<String> = vec![];

        for i in 0..s.len() {
            if s.index(i..i+1) == "1" {
                cnt += 1;
            } else {
                cnt -=1;
                if cnt == 0 {
                    subs.push(format!("1{}0", Solution::make_largest_special(s.index(left + 1..i).to_owned())));
                    left = i + 1;
                }
            }
        }

        subs.sort_by(|a, b| {b.cmp(a)});

        subs.join("")
    }
}

struct Solution;

fn main() {
    let string = Solution::make_largest_special("11011000".to_owned());
    assert_eq!("11100100".to_owned(), string);
}