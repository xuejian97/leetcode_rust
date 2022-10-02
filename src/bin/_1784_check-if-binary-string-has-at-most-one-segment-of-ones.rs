/*
Given a binary string s without leading zeros, return true if s contains at most one contiguous segment of ones. Otherwise, return false.

 

Example 1:

Input: s = "1001"
Output: false
Explanation: The ones do not form a contiguous segment.
Example 2:

Input: s = "110"
Output: true
 

Constraints:

1 <= s.length <= 100
s[i] is either '0' or '1'.
s[0] is '1'.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/check-if-binary-string-has-at-most-one-segment-of-ones
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        let mut pre_1_idx = -1;
        for (i, &x) in s.as_bytes().iter().enumerate() {
            if x == '1' as u8 {
                if pre_1_idx == -1 || pre_1_idx + 1 == i as i32  {
                    pre_1_idx = i as i32;
                } else {
                    return false;
                }
            }
        }

        pre_1_idx != -1
    }
}

struct Solution;

fn main(){

}