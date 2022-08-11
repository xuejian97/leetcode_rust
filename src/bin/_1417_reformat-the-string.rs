/*
You are given an alphanumeric string s. (Alphanumeric string is a string consisting of lowercase English letters and digits).

You have to find a permutation of the string where no letter is followed by another letter and no digit is followed by another digit. That is, no two adjacent characters have the same type.

Return the reformatted string or return an empty string if it is impossible to reformat the string.



Example 1:

Input: s = "a0b1c2"
Output: "0a1b2c"
Explanation: No two adjacent characters have the same type in "0a1b2c". "a0b1c2", "0a1b2c", "0c2a1b" are also valid permutations.
Example 2:

Input: s = "leetcode"
Output: ""
Explanation: "leetcode" has only characters so we cannot separate them by digits.
Example 3:

Input: s = "1229857369"
Output: ""
Explanation: "1229857369" has only digits so we cannot separate them by characters.


Constraints:

1 <= s.length <= 500
s consists of only lowercase English letters and/or digits.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reformat-the-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn reformat(s: String) -> String {
        let mut digit_vec = vec![];
        let mut letter_vec = vec![];
        for x in s.split("") {
            if x == "" { continue; }
            match x.parse::<i32>() {
                Ok(d) => digit_vec.push(d.to_string()),
                Err(_) => letter_vec.push(x.to_string()),
            }
        }

        if (digit_vec.len() as i32 - letter_vec.len() as i32).abs() > 1 { return "".to_string(); }

        if digit_vec.len() > letter_vec.len() {
            letter_vec.push("".to_string());
            digit_vec.iter().zip(letter_vec.iter())
        } else if letter_vec.len() > digit_vec.len() {
            digit_vec.push("".to_string());
            letter_vec.iter().zip(digit_vec.iter())
        } else {
            digit_vec.iter().zip(letter_vec.iter())
        }.flat_map(|(a, b)| {
            return vec![a.to_string(), b.to_string()];
        }).collect::<Vec<String>>().join("")
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "0a1b2c".to_string());
    assert_eq!(Solution::reformat("leetcode".to_string()), "".to_string());
    assert_eq!(Solution::reformat("1229857369".to_string()), "".to_string());
    assert_eq!(Solution::reformat("a0b1c2a".to_string()), "a0b1c2a".to_string());
    assert_eq!(Solution::reformat("a0b1c22".to_string()), "0a1b2c2".to_string());
}