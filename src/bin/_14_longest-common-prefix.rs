/*
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".

 

Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
 

Constraints:

1 <= strs.length <= 200
0 <= strs[i].length <= 200
strs[i] consists of only lowercase English letters.


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-common-prefix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut cnt = strs[0].as_bytes();
        for idx in 1..strs.len() {
            let str = strs[idx].clone();
            if str.len() < cnt.len() {
                cnt = &cnt[0..str.len()];
            }
            for (idx, &v) in str.as_bytes().iter().enumerate() {
                if idx < cnt.len() && v == cnt[idx] { continue; }
                cnt = &cnt[0..idx];
                break;
            }
        }

        String::from_utf8(cnt.to_vec()).unwrap()
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["flower".to_owned(), "flow".to_owned(), "flight".to_owned()]),
        "fl".to_owned()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["ab".to_owned(), "a".to_owned()]),
        "a".to_owned()
    );
}