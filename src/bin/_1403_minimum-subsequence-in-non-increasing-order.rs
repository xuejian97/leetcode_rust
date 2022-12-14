/*
Given the array nums, obtain a subsequence of the array whose sum of elements is strictly greater than the sum of the nonincluded elements in such subsequence.

If there are multiple solutions, return the subsequence with minimum size and if there still exist multiple solutions, return the subsequence with the maximum total sum of all its elements. A subsequence of an array can be obtained by erasing some (possibly zero) elements from the array.

Note that the solution with the given constraints is guaranteed to beunique. Also return the answer sorted in non-increasing order.



Example 1:

Input: nums = [4,3,10,9,8]
Output: [10,9]
Explanation: The subsequences [10,9] and [10,8] are minimal such that the sum of their elements is strictly greater than the sum of elements not included, however, the subsequence [10,9] has the maximum total sum of its elements.
Example 2:

Input: nums = [4,4,7,6,7]
Output: [7,7,6]
Explanation: The subsequence [7,7] has the sum of its elements equal to 14 which is not strictly greater than the sum of elements not included (14 = 4 + 4 + 6). Therefore, the subsequence [7,6,7] is the minimal satisfying the conditions. Note the subsequence has to returned in non-decreasing order.
Example 3:

Input: nums = [6]
Output: [6]


Constraints:

1 <= nums.length <= 500
1 <= nums[i] <= 100

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/minimum-subsequence-in-non-increasing-order
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn min_subsequence(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by(|&a, &b| { b.cmp(&a) });
        let mut sum: i32 = nums.iter().sum();
        let mut subsequence_sum = 0;
        for i in 0..nums.len() {
            sum -= nums[i];
            subsequence_sum += nums[i];

            if subsequence_sum > sum {
                return nums[0..=i].to_vec();
            }
        }

        vec![]
    }
}

struct Solution;

fn main() {
    let vec0 = vec![4, 3, 10, 9, 8];
    let vec1 = vec![4, 4, 7, 6, 7];
    let vec2 = vec![6];

    let vec0_res = Solution::min_subsequence(vec0);
    let vec1_res = Solution::min_subsequence(vec1);
    let vec2_res = Solution::min_subsequence(vec2);

    assert_eq!(vec0_res, vec![10, 9]);
    assert_eq!(vec1_res, vec![7, 7, 6]);
    assert_eq!(vec2_res, vec![6]);
}