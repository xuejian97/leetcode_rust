/*
Given astring sof zeros and ones, return the maximum score after splitting the string into two non-empty substrings (i.e. left substring and right substring).

The score after splitting a string is the number of zeros in the left substring plus the number of ones in the right substring.



Example 1:

Input: s = "011101"
Output: 5 
Explanation: 
All possible ways of splitting s into two non-empty substrings are:
left = "0" and right = "11101", score = 1 + 4 = 5 
left = "01" and right = "1101", score = 1 + 3 = 4 
left = "011" and right = "101", score = 1 + 2 = 3 
left = "0111" and right = "01", score = 1 + 1 = 2 
left = "01110" and right = "1", score = 2 + 1 = 3
Example 2:

Input: s = "00111"
Output: 5
Explanation: When left = "00" and right = "111", we get the maximum score = 2 + 3 = 5
Example 3:

Input: s = "1111"
Output: 3


Constraints:

2 <= s.length <= 500
The string s consists of characters '0' and '1' only.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-score-after-splitting-a-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn max_score(s: String) -> i32 {
        use std::ops::Index;

        let mut vec:Vec<i32> = vec![0];
        let mut value_one_cnt = if s.index(s.len() - 1..) == "1" { 1 } else { 0 };
        for i in 0..s.len() - 1 {
            let x = s.index(i..=i);
            match x {
                "0" => vec.push(vec.last().unwrap() + 1),
                "1" => {
                    vec.push(vec.last().unwrap() - 1);
                    value_one_cnt += 1;
                },
                _ => {}
            }
        }
        vec.remove(0);
        vec.into_iter().max().unwrap() + value_one_cnt
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_score("011101".to_owned()),
        5
    );
    assert_eq!(
        Solution::max_score("00111".to_owned()),
        5
    );
    assert_eq!(
        Solution::max_score("1111".to_owned()),
        3
    );
    assert_eq!(
        Solution::max_score("00".to_owned()),
        1
    );
}