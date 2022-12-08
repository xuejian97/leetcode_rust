/*
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.



Example 1:


Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
Example 2:

Input: height = [4,2,0,3,2,5]
Output: 9


Constraints:

n == height.length
1 <= n <= 2 * 104
0 <= height[i] <= 105
 */
use std::cmp::{max, min};

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_max = 0;
        let mut left = vec![0; height.len()];
        for i in 0..height.len() {
            left_max = max(height[i], left_max);
            left[i] = left_max - height[i];
        }

        let mut right_max = 0;
        let mut right = vec![0; height.len()];
        for i in (0..height.len()).rev() {
            right_max = max(height[i], right_max);
            right[i] = right_max - height[i];
        }

        let mut ans = 0;

        for i in 0..height.len() {
            ans += min(left[i], right[i]);
        }

        ans
    }
}

struct Solution;

fn main() {}