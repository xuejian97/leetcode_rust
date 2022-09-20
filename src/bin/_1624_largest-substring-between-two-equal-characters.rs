/*
Given a string s, return the length of the longest substring between two equal characters, excluding the two characters. If there is no such substring return -1.

A substring is a contiguous sequence of characters within a string.

 

Example 1:

Input: s = "aa"
Output: 0
Explanation: The optimal substring here is an empty substring between the two 'a's.
Example 2:

Input: s = "abca"
Output: 2
Explanation: The optimal substring here is "bc".
Example 3:

Input: s = "cbzxy"
Output: -1
Explanation: There are no characters that appear twice in s.
 

Constraints:

1 <= s.length <= 300
s contains only lowercase English letters.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/largest-substring-between-two-equal-characters
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        use std::collections::HashMap;
        let mut max_length = -1;

        let mut map: HashMap<u8, usize> = HashMap::new();
        for (index, letter) in s.as_bytes().iter().enumerate() {
            match map.get(letter) {
                Some(first_index) => {
                    let length = index as i32 - *first_index as i32 - 1;
                    if length > max_length {
                        max_length = length
                    }
                }
                None => {
                    map.insert(*letter, index);
                }
            }
        }

        max_length
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::max_length_between_equal_characters("aa".to_string()), 0);
    assert_eq!(Solution::max_length_between_equal_characters("abca".to_string()), 2);
    assert_eq!(Solution::max_length_between_equal_characters("cbzxy".to_string()), -1);
    assert_eq!(Solution::max_length_between_equal_characters("cabbac".to_string()), 4);
}