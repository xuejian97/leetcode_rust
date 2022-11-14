/*
You are given an integer array nums.

You should move each element of nums into one of the two arrays A and B such that A and B are non-empty, and average(A) == average(B).

Return true if it is possible to achieve that and false otherwise.

Note that for an array arr, average(arr) is the sum of all the elements of arr over the length of arr.



Example 1:

Input: nums = [1,2,3,4,5,6,7,8]
Output: true
Explanation: We can split the array into [1,4,5,8] and [2,3,6,7], and both of them have an average of 4.5.
Example 2:

Input: nums = [3,1]
Output: false


Constraints:

1 <= nums.length <= 30
0 <= nums[i] <= 104
 */

use std::collections::HashSet;

impl Solution {
    pub fn split_array_same_average(nums: Vec<i32>) -> bool {
        if nums.len() < 2 { return false; }
        let fix_nums: Vec<i32> = nums.iter().map(|n| { return *n * nums.len() as i32; }).collect();
        let fix_nums_sum: i32 = fix_nums.iter().sum();
        let fix_nums_avg: i32 = fix_nums_sum / nums.len() as i32;
        let fix_nums: Vec<i32> = fix_nums.iter().map(|n| { return *n - fix_nums_avg; }).collect();
        let m = nums.len() / 2;
        let left_fix_nums = &fix_nums[..m];
        let right_fix_nums = &fix_nums[m..];
        let mut set = HashSet::new();
        for i in 1..1 << m {
            let mut cur_sum = 0;
            for (index, &x) in format!("{i:>0m$b}").as_bytes().iter().enumerate() {
                if x as char == '1' {
                    cur_sum += left_fix_nums[index];
                }
            };
            if cur_sum == 0 { return true; }
            set.insert(cur_sum);
        }

        let rsum: i32 = right_fix_nums.iter().sum();
        let n = nums.len() - m;
        for i in 1..1 << n {
            let mut cur_sum = 0;
            for (index, &x) in format!("{i:>0n$b}").as_bytes().iter().enumerate() {
                if x as char == '1' {
                    cur_sum += right_fix_nums[index];
                }
            };
            if cur_sum == 0 || (rsum != cur_sum && set.contains(&-cur_sum)) { return true; }
        }

        false
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::split_array_same_average(vec![3,1,2]), true)
}