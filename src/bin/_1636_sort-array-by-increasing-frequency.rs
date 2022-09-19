/*
Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.

Return the sorted array.

 

Example 1:

Input: nums = [1,1,2,2,2,3]
Output: [3,1,1,2,2,2]
Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
Example 2:

Input: nums = [2,3,1,3,2]
Output: [1,3,3,2,2]
Explanation: '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.
Example 3:

Input: nums = [-1,1,-6,4,5,-6,1,4,1]
Output: [5,-1,4,4,-6,-6,1,1,1]
 

Constraints:

1 <= nums.length <= 100
-100 <= nums[i] <= 100

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/sort-array-by-increasing-frequency
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        use std::cmp::Ordering;

        let mut cnt_map: HashMap<i32, i32> = HashMap::new();

        for x in &nums {
            match cnt_map.get_mut(x) {
                Some(cnt) => { *cnt += 1 }
                None => { cnt_map.insert(*x, 1); }
            }
        }

        nums.sort_by(|a, b| {
            let ordering = cnt_map.get(a).cmp(&cnt_map.get(b));
            if ordering == Ordering::Equal {
                return b.cmp(a);
            }
            return ordering;
        });

        nums
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3]), vec![3, 1, 1, 2, 2, 2]);
    assert_eq!(Solution::frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
    assert_eq!(Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]), vec![5, -1, 4, 4, -6, -6, 1, 1, 1]);
}