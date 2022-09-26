/*
An integer x is a good if after rotating each digit individually by 180 degrees, we get a valid number that is different from x. Each digit must be rotated - we cannot choose to leave it alone.

A number is valid if each digit remains a digit after rotation. For example:

0, 1, and 8 rotate to themselves,
2 and 5 rotate to each other (in this case they are rotated in a different direction, in other words, 2 or 5 gets mirrored),
6 and 9 rotate to each other, and
the rest of the numbers do not rotate to any other number and become invalid.
Given an integer n, return the number of good integers in the range [1, n].

 

Example 1:

Input: n = 10
Output: 4
Explanation: There are four good numbers in the range [1, 10] : 2, 5, 6, 9.
Note that 1 and 10 are not good numbers, since they remain unchanged after rotating.
Example 2:

Input: n = 1
Output: 0
Example 3:

Input: n = 2
Output: 1
 

Constraints:

1 <= n <= 104

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/rotated-digits
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn rotated_digits(mut n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            let i_str = i.to_string();
            if (i_str.contains("2") || i_str.contains("5") || i_str.contains("6") || i_str.contains("9"))
                && !i_str.contains("3")
                && !i_str.contains("4")
                && !i_str.contains("7")
            {
                ans += 1;
            }
        }
        ans
    }
    pub fn get_good_num_cnt(num: i32) -> i32 {
        if num < 2 {
            0
        } else if num < 5 {
            1
        } else if num < 6 {
            2
        } else if num < 9 {
            3
        } else if num < 10 {
            4
        } else {
            0
        }
    }
}

struct Solution;

fn main() {
    let i = Solution::rotated_digits(857);
    println!("{}", i);
}