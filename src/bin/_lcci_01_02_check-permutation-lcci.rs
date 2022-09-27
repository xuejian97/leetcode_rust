/*
Given two strings,write a method to decide if one is a permutation of the other.

Example 1:

Input: s1 = "abc", s2 = "bca"
Output: true
Example 2:

Input: s1 = "abc", s2 = "bad"
Output: false
Note:

0 <= len(s1) <= 100
0 <= len(s2) <= 100

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/check-permutation-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn check_permutation(s1: String, s2: String) -> bool {
        use std::collections::HashMap;
        let s1_bytes = s1.as_bytes();
        let s2_bytes = s2.as_bytes();

        let mut dict: HashMap<u8, i32> = HashMap::new();

        for x in s1_bytes {
            match dict.get_mut(x) {
                Some(cnt) => { *cnt += 1 }
                None => { dict.insert(*x, 1); }
            }
        }

        for x in s2_bytes {
            match dict.get_mut(x) {
                Some(cnt) => {
                    *cnt -= 1;
                    if *cnt == 0 {
                        dict.remove(x);
                    }
                }
                None => {
                    dict.insert(*x, -1);
                }
            }
        }

        dict.len() == 0
    }
}

struct Solution;

fn main() {}