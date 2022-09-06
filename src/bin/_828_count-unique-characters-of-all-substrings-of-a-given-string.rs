/*
Let's define a function countUniqueChars(s) that returns the number of unique characters on s.

For example, calling countUniqueChars(s) if s = "LEETCODE" then "L", "T", "C", "O", "D" are the unique characters since they appear only once in s, therefore countUniqueChars(s) = 5.
Given a string s, return the sum of countUniqueChars(t) where t is a substring of s. The test cases are generated such that the answer fits in a 32-bit integer.

Notice that some substrings can be repeated so in this case you have to count the repeated ones too.

 

Example 1:

Input: s = "ABC"
Output: 10
Explanation: All possible substrings are: "A","B","C","AB","BC" and "ABC".
Every substring is composed with only unique letters.
Sum of lengths of all substring is 1 + 1 + 1 + 2 + 2 + 3 = 10
Example 2:

Input: s = "ABA"
Output: 8
Explanation: The same as example 1, except countUniqueChars("ABA") = 1.
Example 3:

Input: s = "LEETCODE"
Output: 92
 

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/count-unique-characters-of-all-substrings-of-a-given-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<u8, Vec<i32>> = HashMap::new();
        for (i, x) in s.as_bytes().iter().enumerate() {
            if let None = map.get(x) {
                map.insert(*x, vec![-1]);
            }
            map.get_mut(x).unwrap().push(i as i32);
        }

        let mut res = 0;

        for (_, v) in map.iter_mut() {
            v.push(s.len() as i32);
            for i in 1..v.len() - 1 {
                res += (v[i] - v[i - 1]) * (v[i + 1] - v[i]);
            }
        }

        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
    assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
    assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
}