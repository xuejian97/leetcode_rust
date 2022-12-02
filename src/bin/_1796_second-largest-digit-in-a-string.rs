/*
Given an alphanumeric string s, return the second largest numerical digit that appears in s, or -1 if it does not exist.

An alphanumeric string is a string consisting of lowercase English letters and digits.



Example 1:

Input: s = "dfa12321afd"
Output: 2
Explanation: The digits that appear in s are [1, 2, 3]. The second largest digit is 2.
Example 2:

Input: s = "abc1111"
Output: -1
Explanation: The digits that appear in s are [1]. There is no second largest digit.


Constraints:

1 <= s.length <= 500
s consists of only lowercase English letters and/or digits.
 */
impl Solution {
    pub fn second_highest(s: String) -> i32 {
        let mut nums: Vec<i32> = s.as_bytes().iter()
            .filter(|&b| { *b >= 48_u8 && *b <= 57_u8 })
            .map(|b| { *b as i32 - 48 })
            .collect();
        if nums.len() <= 1 { return -1; }
        nums.sort_unstable();
        let max = nums.last().unwrap();
        for i in (0..nums.len()).rev() {
            if nums[i] != *max { return nums[i]; }
        }
        -1
    }
}

struct Solution;

fn main() {}