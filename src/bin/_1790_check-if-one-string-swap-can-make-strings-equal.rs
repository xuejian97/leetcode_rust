/*
You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose two indices in a string (not necessarily different) and swap the characters at these indices.

Return true if it is possible to make both strings equal by performing at most one string swap on exactly one of the strings. Otherwise, return false.



Example 1:

Input: s1 = "bank", s2 = "kanb"
Output: true
Explanation: For example, swap the first character with the last character of s2 to make "bank".
Example 2:

Input: s1 = "attack", s2 = "defend"
Output: false
Explanation: It is impossible to make them equal with one string swap.
Example 3:

Input: s1 = "kelb", s2 = "kelb"
Output: true
Explanation: The two strings are already equal, so no string swap operation is required.


Constraints:

1 <= s1.length, s2.length <= 100
s1.length == s2.length
s1 and s2 consist of only lowercase English letters.
 */

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        if s1 == s2 { return true; }
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();
        let mut s1_1st_diff = 0;
        let mut s1_2ed_diff = 0;
        let mut s2_1st_diff = 0;
        let mut s2_2ed_diff = 0;
        let mut idx = 0;
        for i in 0..s1_bytes.len() {
            if s1_bytes[i] != s2_bytes[i] {
                idx += 1;
                if idx == 1 {
                    s1_1st_diff = s1_bytes[i];
                    s2_1st_diff = s2_bytes[i];
                } else if idx == 2 {
                    s1_2ed_diff = s1_bytes[i];
                    s2_2ed_diff = s2_bytes[i];
                } else {
                    return false;
                }
            }
        }

        if idx != 2 { return false; }
        s1_1st_diff == s2_2ed_diff && s1_2ed_diff == s2_1st_diff
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::are_almost_equal("bank".to_owned(), "kanb".to_owned()), true);
    assert_eq!(Solution::are_almost_equal("attack".to_owned(), "defend".to_owned()), false);
    assert_eq!(Solution::are_almost_equal("kelb".to_owned(), "kelb".to_owned()), true);
}