/*
There is a room with n bulbs labeled from 1 to n that all are turned on initially, and four buttons on the wall. Each of the four buttons has a different functionality where:

Button 1: Flips the status of all the bulbs.
Button 2: Flips the status of all the bulbs with even labels (i.e., 2, 4, ...).
Button 3: Flips the status of all the bulbs with odd labels (i.e., 1, 3, ...).
Button 4: Flips the status of all the bulbs with a label j = 3k + 1 where k = 0, 1, 2, ... (i.e., 1, 4, 7, 10, ...).
You must make exactly presses button presses in total. For each press, you may pick any of the four buttons to press.

Given the two integers n and presses, return the number of different possible statuses after performing all presses button presses.

 

Example 1:

Input: n = 1, presses = 1
Output: 2
Explanation: Status can be:
- [off] by pressing button 1
- [on] by pressing button 2
Example 2:

Input: n = 2, presses = 1
Output: 3
Explanation: Status can be:
- [off, off] by pressing button 1
- [on, off] by pressing button 2
- [off, on] by pressing button 3
Example 3:

Input: n = 3, presses = 1
Output: 4
Explanation: Status can be:
- [off, off, off] by pressing button 1
- [off, on, off] by pressing button 2
- [on, off, on] by pressing button 3
- [off, on, on] by pressing button 4
 

Constraints:

1 <= n <= 1000
0 <= presses <= 1000


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/bulb-switcher-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn flip_lights(n: i32, presses: i32) -> i32 {
        match n {
            0 => return 0,
            1 => return if presses > 0 { 1 } else { 0 } + 1,
            2 => return if presses > 1 { 1 } else { 0 } + if presses > 0 { 1 } else { 0 } * 2 + 1,
            _ => ()
        }
        match presses {
            0 => return 1,
            1 => return 4,
            2 => return 7,
            _ => ()
        }
        8
    }
}

struct Solution;

fn main() {}