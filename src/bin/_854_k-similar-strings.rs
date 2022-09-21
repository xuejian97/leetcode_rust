/*
Strings s1 and s2 are k-similar (for some non-negative integer k) if we can swap the positions of two letters in s1 exactly k times so that the resulting string equals s2.

Given two anagrams s1 and s2, return the smallest k for which s1 and s2 are k-similar.

 

Example 1:

Input: s1 = "ab", s2 = "ba"
Output: 1
Example 2:

Input: s1 = "abc", s2 = "bca"
Output: 2
 

Constraints:

1 <= s1.length <= 20
s2.length == s1.length
s1 and s2 contain only lowercase letters from the set {'a', 'b', 'c', 'd', 'e', 'f'}.
s2 is an anagram of s1.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/k-similar-strings
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        use std::collections::{VecDeque, HashSet};
        use std::ops::Index;

        let s_length = s1.len();
        let mut step = 0;

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((s1, 0_usize));

        while queue.len() > 0 {
            for _ in 0..queue.len() {
                let (cur, mut pos) = queue.pop_front().unwrap();
                if cur == s2 { return step; }
                while pos < s_length && cur.index(pos..=pos) == s2.index(pos..=pos) {
                    pos += 1;
                }
                for j in pos + 1..s_length {
                    if s2.index(j..=j) == cur.index(j..=j) { continue; }
                    if s2.index(pos..=pos) == cur.index(j..=j) {
                        let next = Solution::swap(cur.clone(), pos, j);
                        if !visited.contains(&next) {
                            visited.insert(next.clone());
                            queue.push_back((next, pos + 1))
                        }
                    }
                }
            }
            step += 1;
        }

        step
    }

    fn swap(cur: String, i: usize, j: usize) -> String {
        let mut arr = cur.as_bytes().to_vec();
        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
        String::from_utf8(arr).unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::k_similarity("ab".to_owned(), "ba".to_owned()), 1);
    assert_eq!(Solution::k_similarity("abc".to_owned(), "bca".to_owned()), 2);
}