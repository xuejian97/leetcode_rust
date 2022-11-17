/*
Given a string s and an array of strings words, return the number of words[i] that is a subsequence of s.

A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.

For example, "ace" is a subsequence of "abcde".


Example 1:

Input: s = "abcde", words = ["a","bb","acd","ace"]
Output: 3
Explanation: There are three strings in words that are a subsequence of s: "a", "acd", "ace".
Example 2:

Input: s = "dsahjpjauf", words = ["ahjpjau","ja","ahbwzgqnuk","tnmlanowax"]
Output: 2


Constraints:

1 <= s.length <= 5 * 104
1 <= words.length <= 5000
1 <= words[i].length <= 50
s and words[i] consist of only lowercase English letters.
 */

use std::collections::HashMap;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let all = words.len();
        let mut map: HashMap<u8, Vec<Vec<u8>>> = HashMap::new();
        for x in words {
            let bytes = x.as_bytes().to_vec();
            if let Some(vec) = map.get_mut(&bytes[0]) {
                vec.push(bytes);
            } else {
                map.insert(bytes[0], vec![bytes]);
            }
        }
        for x in s.as_bytes() {
            if !map.contains_key(x) { continue; }
            let vec = map.get_mut(x).unwrap();
            let mut new_vec = vec![];
            for bytes in vec {
                let i = bytes.remove(0);
                if bytes.len() == 0 { continue; }
                new_vec.push(bytes.clone());
            }
            map.remove(x);

            if new_vec.len() == 0 {
                continue;
            }

            for bytes in new_vec {
                if let Some(vec) = map.get_mut(&bytes[0]) {
                    vec.push(bytes);
                } else {
                    map.insert(bytes[0], vec![bytes]);
                }
            }
        }

        let left: i32 = map.iter().map(|(_, vec)| { return vec.len() as i32; }).sum();
        all as i32 - left
    }
}

struct Solution;

fn main() {}