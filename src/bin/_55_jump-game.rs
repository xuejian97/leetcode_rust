/*
You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.



Example 1:

Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
Example 2:

Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.


Constraints:

1 <= nums.length <= 104
0 <= nums[i] <= 105
 */
use std::cmp::max;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // Solution::can_jump_dp(nums)
        Solution::can_jump_greedy(nums)
    }

    pub fn can_jump_dp(nums: Vec<i32>) -> bool {
        let mut dp = vec![false; nums.len()];
        dp[0] = true;

        for i in 0..nums.len() - 1 {
            if dp[i] == false { continue; }
            for j in i + 1..=(i + nums[i] as usize) {
                if j >= nums.len() - 1 {
                    return true;
                }
                dp[j] = true;
            }
        }

        *dp.last().unwrap()
    }

    pub fn can_jump_greedy(nums: Vec<i32>) -> bool {
        let mut can_jump_index = nums[0];

        for i in 0..nums.len() {
            if i as i32 > can_jump_index { break; }
            can_jump_index = max(can_jump_index, i as i32 + nums[i]);
            if can_jump_index >= nums.len() as i32 - 1 { return true; }
        }

        false
    }
}

struct Solution;

fn main() {}