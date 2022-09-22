/*
Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.

Notice that the solution set must not contain duplicate triplets.

 

Example 1:

Input: nums = [-1,0,1,2,-1,-4]
Output: [[-1,-1,2],[-1,0,1]]
Explanation: 
nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
The distinct triplets are [-1,0,1] and [-1,-1,2].
Notice that the order of the output and the order of the triplets does not matter.
Example 2:

Input: nums = [0,1,1]
Output: []
Explanation: The only possible triplet does not sum up to 0.
Example 3:

Input: nums = [0,0,0]
Output: [[0,0,0]]
Explanation: The only possible triplet sums up to 0.
 

Constraints:

3 <= nums.length <= 3000
-105 <= nums[i] <= 105

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/3sum
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;

        if nums.len() < 3 { return vec![]; }

        nums.sort_unstable();

        let mut ans = vec![];

        let mut i = 0;
        while i < nums.len() - 2 {
            let mut left_index = i + 1;
            let mut right_index = nums.len() - 1;
            let cur = nums[i];
            while left_index < right_index {
                match 0.cmp(&(cur + nums[left_index] + nums[right_index])) {
                    Ordering::Greater => {
                        left_index += 1;
                        while left_index <= right_index && nums[left_index] == nums[left_index - 1] {
                            left_index += 1;
                        }
                    }
                    Ordering::Less => {
                        right_index -= 1;
                        while right_index >= left_index && nums[right_index] == nums[right_index + 1] {
                            right_index -= 1;
                        }
                    }
                    Ordering::Equal => {
                        ans.push(vec![cur, nums[left_index], nums[right_index]]);
                        right_index -= 1;
                        while right_index >= left_index && nums[right_index] == nums[right_index + 1] {
                            right_index -= 1;
                        }
                        left_index += 1;
                        while left_index <= right_index && nums[left_index] == nums[left_index - 1] {
                            left_index += 1;
                        }
                    }
                }
            }

            i += 1;
            while i < nums.len() - 2 && nums[i] == nums[i - 1] {
                i += 1;
            }
        }

        ans
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::three_sum(vec![1, -1, -1, 0]),
        vec![[-1, 0, 1]]
    );
    // assert_eq!(
    //     Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
    //     vec![[-1, -1, 2], [-1, 0, 1]]
    // );
    // assert_eq!(
    //     Solution::three_sum(vec![0, 0, 0, 0]),
    //     vec![[0, 0, 0]]
    // );
    // let vec1: Vec<Vec<i32>> = vec![];
    // assert_eq!(
    //     Solution::three_sum(vec![0, 1, 1]),
    //     vec1
    // );
}