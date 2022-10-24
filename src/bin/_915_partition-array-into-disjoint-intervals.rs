/*
Given an integer array nums, partition it into two (contiguous) subarrays left and right so that:

Every element in left is less than or equal to every element in right.
left and right are non-empty.
left has the smallest possible size.
Return the length of left after such a partitioning.

Test cases are generated such that partitioning exists.



Example 1:

Input: nums = [5,0,3,8,6]
Output: 3
Explanation: left = [5,0,3], right = [8,6]
Example 2:

Input: nums = [1,1,1,0,6,12]
Output: 4
Explanation: left = [1,1,1,0], right = [6,12]


Constraints:

2 <= nums.length <= 105
0 <= nums[i] <= 106
There is at least one valid answer for the given input.
 */

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut sorted_nums = nums.clone();
        sorted_nums.sort_unstable();

        let mut ans = 0;
        let mut map = HashMap::new();

        for x in nums {
            match map.get_mut(&x) {
                Some(cnt) => {
                    *cnt += 1;
                }
                None => {
                    map.insert(x, 1);
                }
            }

            while ans < sorted_nums.len() && map.contains_key(&sorted_nums[ans]) {
                if let Some(cnt) = map.get_mut(&sorted_nums[ans]) {
                    *cnt -= 1;
                    if cnt == &0 {
                        map.remove(&sorted_nums[ans]);
                    }
                    ans += 1;
                }

            }

            if map.len() == 0 { break; }
        }

        ans as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    assert_eq!(Solution::partition_disjoint(vec![90, 47, 69, 10, 43, 92, 31, 73, 61, 97]), 9);
}