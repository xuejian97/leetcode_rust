/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.

Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

 

Example 1:

Input: x = 123
Output: 321
Example 2:

Input: x = -123
Output: -321
Example 3:

Input: x = 120
Output: 21
 

Constraints:

-231 <= x <= 231 - 1

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/reverse-integer
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let i = if x > 0 { 1 } else { -1 };
        let mut res = 0i64;
        let mut x = x.abs() as i64;
        while x > 0 {
            res = res * 10 + x % 10;
            x /= 10;
        }

        let res = res * i;
        if res < i32::MIN as i64 || res > i32::MAX as i64 {
            0
        } else {
            res as i32
        }
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::reverse(123));
    println!("{}", Solution::reverse(-123));
    println!("{}", Solution::reverse(120));
    println!("{}", Solution::reverse(0));
    println!("{}", Solution::reverse(1534236469));
}