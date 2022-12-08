/*
Given an integer array nums, find the
subarray
 which has the largest sum and return its sum.



Example 1:

Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
Example 2:

Input: nums = [1]
Output: 1
Example 3:

Input: nums = [5,4,-1,7,8]
Output: 23


Constraints:

1 <= nums.length <= 105
-104 <= nums[i] <= 104


Follow up: If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
 */
use std::cmp::max;

impl Solution {
    pub fn max_sub_array(mut nums: Vec<i32>) -> i32 {
        let mut max_sum = nums[0];
        for i in 1..nums.len() {
            nums[i] = max(nums[i - 1] + nums[i], nums[i]);
            max_sum = max(max_sum, nums[i]);
        }
        max_sum
    }
}

struct Solution;

fn main() {}