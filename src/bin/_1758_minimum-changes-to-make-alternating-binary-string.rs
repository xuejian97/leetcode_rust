/*
You are given a string s consisting only of the characters '0' and '1'. In one operation, you can change any '0' to '1' or vice versa.

The string is called alternating if no two adjacent characters are equal. For example, the string "010" is alternating, while the string "0100" is not.

Return the minimum number of operations needed to make s alternating.



Example 1:

Input: s = "0100"
Output: 1
Explanation: If you change the last character to '1', s will be "0101", which is alternating.
Example 2:

Input: s = "10"
Output: 0
Explanation: s is already alternating.
Example 3:

Input: s = "1111"
Output: 2
Explanation: You need two operations to reach "0101" or "1010".


Constraints:

1 <= s.length <= 104
s[i] is either '0' or '1'.
 */
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut res_vec = vec![0, 0];
        let fix_vec = vec![0, 1];

        for (i, &x) in s.as_bytes().iter().enumerate() {
            for fix in &fix_vec {
                let target = (i + fix) % 2;
                if target as u8 != x - 48 {
                    res_vec[*fix] += 1;
                }
            }
        }
        res_vec[0].min(res_vec[1])
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::min_operations("0100".to_owned()), 1);
}