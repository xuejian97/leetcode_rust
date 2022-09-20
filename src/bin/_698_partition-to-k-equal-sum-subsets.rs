/*
Given an integer array nums and an integer k, return true if it is possible to divide this array into k non-empty subsets whose sums are all equal.

 

Example 1:

Input: nums = [4,3,2,3,5,2,1], k = 4
Output: true
Explanation: It is possible to divide it into 4 subsets (5), (1, 4), (2,3), (2,3) with equal sums.
Example 2:

Input: nums = [1,2,3,4], k = 3
Output: false
 

Constraints:

1 <= k <= nums.length <= 16
1 <= nums[i] <= 104
The frequency of each element is in the range [1, 4].

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/partition-to-k-equal-sum-subsets
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn can_partition_k_subsets(mut nums: Vec<i32>, mut k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        let remainder = sum % k;
        if remainder != 0 { return false; }

        let sub_sum = sum / k;
        nums.sort_unstable();
        if *nums.last().unwrap() > sub_sum { return false; }
        while nums.len() > 0 && *nums.last().unwrap() == sub_sum {
            nums.pop();
            k -= 1;
        }

        Solution::partition(&mut vec![0; k as usize], &nums, nums.len() as i32 - 1, sub_sum)
    }

    pub fn partition(sub_sets: &mut Vec<i32>, nums: &Vec<i32>, index: i32, target: i32) -> bool {
        if index < 0 {
            return true;
        }
        let selected = nums[index as usize];
        for i in 0..sub_sets.len() {
            if sub_sets[i] + selected <= target {
                sub_sets[i] += selected;
                if Solution::partition(sub_sets, nums, index as i32 - 1, target) {
                    return true;
                }
                sub_sets[i] -= selected;
            }
        }
        false
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_partition_k_subsets(vec![4, 3, 2, 3, 5, 2, 1], 4), true);
    assert_eq!(Solution::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
    assert_eq!(Solution::can_partition_k_subsets(vec![2, 2, 2, 2, 3, 4, 5], 4), false);
}