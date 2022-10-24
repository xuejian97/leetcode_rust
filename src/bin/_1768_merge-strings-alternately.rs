/*
You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

Return the merged string.



Example 1:

Input: word1 = "abc", word2 = "pqr"
Output: "apbqcr"
Explanation: The merged string will be merged as so:
word1:  a   b   c
word2:    p   q   r
merged: a p b q c r
Example 2:

Input: word1 = "ab", word2 = "pqrs"
Output: "apbqrs"
Explanation: Notice that as word2 is longer, "rs" is appended to the end.
word1:  a   b
word2:    p   q   r   s
merged: a p b q   r   s
Example 3:

Input: word1 = "abcd", word2 = "pq"
Output: "apbqcd"
Explanation: Notice that as word1 is longer, "cd" is appended to the end.
word1:  a   b   c   d
word2:    p   q
merged: a p b q c   d


Constraints:

1 <= word1.length, word2.length <= 100
word1 and word2 consist of lowercase English letters.
 */

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut ans = vec![];
        let (mut index_1, mut index_2) = (0, 0);
        while index_1 < word1.len() || index_2 < word2.len() {
            if index_1 < word1.len() {
                ans.push(&word1[index_1..=index_1]);
                index_1 += 1;
            }
            if index_2 < word2.len() {
                ans.push(&word2[index_2..=index_2]);
                index_2 += 1;
            }
        }

        ans.join("")
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::merge_alternately("abc".to_owned(), "pqr".to_owned()), "apbqcr".to_owned());
}