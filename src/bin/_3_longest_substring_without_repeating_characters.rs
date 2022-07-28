/*
Given a string s, find the length of the longest substring without repeating characters.



Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.


Constraints:

0 <= s.length <= 5 * 104
s consists of English letters, digits, symbols and spaces.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-substring-without-repeating-characters
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 { return 0; }

        let mut map:HashMap<&str, usize> = HashMap::new();
        let mut max = 0;
        let mut left = 0usize;

        for i in 0..s.len() {
            let char = &s[i..=i];
            match map.get(char) {
                Some(idx) => {
                    left = left.max(idx + 1);
                }
                None=> {}
            }
            map.insert(char, i);
            max = max.max(i - left + 1);
        }

        max as i32
    }
}
fn main(){
    println!("{}", Solution::length_of_longest_substring(String::from("abba")));
}