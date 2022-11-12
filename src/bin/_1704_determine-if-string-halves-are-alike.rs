/*
You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.

Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.

Return true if a and b are alike. Otherwise, return false.



Example 1:

Input: s = "book"
Output: true
Explanation: a = "bo" and b = "ok". a has 1 vowel and b has 1 vowel. Therefore, they are alike.
Example 2:

Input: s = "textbook"
Output: false
Explanation: a = "text" and b = "book". a has 1 vowel whereas b has 2. Therefore, they are not alike.
Notice that the vowel o is counted twice.


Constraints:

2 <= s.length <= 1000
s.length is even.
s consists of uppercase and lowercase letters.
 */
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        use std::collections::HashSet;

        let half_len = s.len() / 2;
        let mut left_cnt = 0;
        let mut right_cnt = 0;

        let mut vowels = HashSet::new();
        for x in vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'] {
            vowels.insert(x as u8);
        }
        let s_bytes = s.as_bytes();
        for i in 0..half_len {
            if vowels.contains(&s_bytes[i]) { left_cnt += 1; }
            if vowels.contains(&s_bytes[half_len + i]) { right_cnt += 1; }
        }

        left_cnt == right_cnt
    }
}

struct Solution;

fn main() {}