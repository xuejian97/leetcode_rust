/*
Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.

The overall run time complexity should be O(log (m+n)).



Example 1:

Input: nums1 = [1,3], nums2 = [2]
Output: 2.00000
Explanation: merged array = [1,2,3] and median is 2.
Example 2:

Input: nums1 = [1,2], nums2 = [3,4]
Output: 2.50000
Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.


Constraints:

nums1.length == m
nums2.length == n
0 <= m <= 1000
0 <= n <= 1000
1 <= m + n <= 2000
-106 <= nums1[i], nums2[i] <= 106

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/median-of-two-sorted-arrays
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */
struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut result: f64 = 1.0;
        let mut arr = vec![nums1, nums2].concat();
        arr.sort();
        let mid = arr.len() / 2;
        match arr.len() % 2 {
            1 => result = arr[mid] as f64,
            0 => result = (arr[mid - 1] + arr[mid]) as f64 / 2 as f64,
            _ => println!("none")
        };
        result
    }

    pub fn _find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let length = nums1.len() + nums2.len();
        let even = length % 2 == 0;
        let mut nums1_index = 0;
        let mut nums2_index = 0;
        let mut last_num = -1;
        let mut i = 0i32;
        while i <= (length / 2) as i32 {
            if i != (length / 2) as i32 {
                if nums1_index == nums1.len() {
                    last_num = nums2[nums2_index];
                    nums2_index += 1;
                } else if nums2_index == nums2.len() {
                    last_num = nums1[nums1_index];
                    nums1_index += 1;
                } else {
                    if nums1[nums1_index] < nums2[nums2_index] {
                        last_num = nums1[nums1_index];
                        nums1_index += 1;
                    } else {
                        last_num = nums2[nums2_index];
                        nums2_index += 1;
                    }
                }

                i += 1;
                continue;
            }

            let end = if nums1_index == nums1.len() { nums2[nums2_index] } else if nums2_index == nums2.len() { nums1[nums1_index] } else { nums1[nums1_index].min(nums2[nums2_index]) } as f64;

            if !even {
                return end;
            } else {
                return (last_num as f64 + end) / 2.0;
            }
        }

        0.0
    }
}

fn main() {
    let nums1 = vec![1, 2];
    let nums2 = vec![7, 8, 9, 10, 11];
    println!("{}", Solution::find_median_sorted_arrays(nums1, nums2));
}