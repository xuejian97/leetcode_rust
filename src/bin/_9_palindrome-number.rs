/*
Given an integer x, return true if x is palindrome integer.

An integer is a palindrome when it reads the same backward as forward.

For example, 121 is a palindrome while 123 is not.
 

Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 

Constraints:

-231 <= x <= 231 - 1
 

Follow up: Could you solve it without converting the integer to a string?

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/palindrome-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() { return false; }
        let mut x = x;
        let mut tail = 0;

        let x_length = x.to_string().len();
        let times = x_length / 2;

        for _ in 0..times {
            tail = tail * 10 + x % 10;
            x /= 10;
        }

        if x_length % 2 == 0 {
            return x == tail;
        } else {
            return x / 10 == tail;
        }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
}