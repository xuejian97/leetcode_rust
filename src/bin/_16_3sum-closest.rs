/*
Given an integer array nums of length n and an integer target, find three integers in nums such that the sum is closest to target.

Return the sum of the three integers.

You may assume that each input would have exactly one solution.

 

Example 1:

Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
Example 2:

Input: nums = [0,0,0], target = 1
Output: 0
Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
 

Constraints:

3 <= nums.length <= 1000
-1000 <= nums[i] <= 1000
-104 <= target <= 104


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/3sum-closest
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        use std::cmp::Ordering;
        let mut ans = None;
        nums.sort_unstable();

        let mut i = 0;
        while i < nums.len() - 2 {
            let mut left_index = i + 1;
            let mut right_index = nums.len() - 1;
            let cur = nums[i];
            while left_index < right_index {
                let cur_sum = cur + nums[left_index] + nums[right_index];
                let cur_diff = (target - cur_sum).abs();
                match ans {
                    None => {
                        ans = Some(cur_sum);
                    }
                    Some(sum) => {
                        if cur_diff < (target - sum).abs() {
                            ans = Some(cur_sum);
                        }
                    }
                }
                match target.cmp(&(cur_sum)) {
                    Ordering::Less => {
                        right_index -= 1;
                        while right_index >= left_index && nums[right_index] == nums[right_index + 1] {
                            right_index -= 1;
                        }
                    }
                    Ordering::Equal => {
                        return cur_sum;
                    }
                    Ordering::Greater => {
                        left_index += 1;
                        while left_index <= right_index && nums[left_index] == nums[left_index - 1] {
                            left_index += 1;
                        }
                    }
                }
            }

            i += 1;
        }

        ans.unwrap()
    }
}

struct Solution;

fn main() {
    // -1,2,1,-4
    // -4 -1 1 2
    // 1

    assert_eq!(Solution::three_sum_closest(vec![-1,2,1,-4], 1), 2);
}