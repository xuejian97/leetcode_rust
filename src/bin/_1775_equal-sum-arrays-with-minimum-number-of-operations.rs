/*
You are given two arrays of integers nums1 and nums2, possibly of different lengths. The values in the arrays are between 1 and 6, inclusive.

In one operation, you can change any integer's value in any of the arrays to any value between 1 and 6, inclusive.

Return the minimum number of operations required to make the sum of values in nums1 equal to the sum of values in nums2. Return -1​​​​​ if it is not possible to make the sum of the two arrays equal.



Example 1:

Input: nums1 = [1,2,3,4,5,6], nums2 = [1,1,2,2,2,2]
Output: 3
Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums2[0] to 6. nums1 = [1,2,3,4,5,6], nums2 = [6,1,2,2,2,2].
- Change nums1[5] to 1. nums1 = [1,2,3,4,5,1], nums2 = [6,1,2,2,2,2].
- Change nums1[2] to 2. nums1 = [1,2,2,4,5,1], nums2 = [6,1,2,2,2,2].
Example 2:

Input: nums1 = [1,1,1,1,1,1,1], nums2 = [6]
Output: -1
Explanation: There is no way to decrease the sum of nums1 or to increase the sum of nums2 to make them equal.
Example 3:

Input: nums1 = [6,6], nums2 = [1]
Output: 3
Explanation: You can make the sums of nums1 and nums2 equal with 3 operations. All indices are 0-indexed.
- Change nums1[0] to 2. nums1 = [2,6], nums2 = [1].
- Change nums1[1] to 2. nums1 = [2,2], nums2 = [1].
- Change nums2[0] to 4. nums1 = [2,2], nums2 = [4].


Constraints:

1 <= nums1.length, nums2.length <= 105
1 <= nums1[i], nums2[i] <= 6
 */
impl Solution {
    pub fn min_operations(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        let nums1_min = nums1.len();
        let nums1_max = nums1.len() * 6;
        let nums2_min = nums2.len();
        let nums2_max = nums2.len() * 6;

        if nums1_max < nums2_max && nums1_max < nums2_min || nums2_max < nums1_max && nums2_max < nums1_min {
            return -1;
        }

        let mut vec = vec![];
        let nums1_sum: i32 = nums1.iter().sum();
        let nums2_sum: i32 = nums2.iter().sum();
        if nums1_sum == nums2_sum { return 0; }
        nums1.sort_unstable();
        nums2.sort_unstable();
        if nums1_sum > nums2_sum {
            vec.push(nums1);
            vec.push(nums2);
        } else {
            vec.push(nums2);
            vec.push(nums1);
        }
        let mut diff = (nums1_sum - nums2_sum).abs();

        let mut big_index = vec[0].len() as i32 - 1;
        let mut small_index = 0_i32;

        let mut cnt = 0;
        while diff > 0 {
            if vec[0][big_index as usize] - 1 > 6 - vec[1][small_index as usize] {
                diff -= (vec[0][big_index as usize] - 1);
                vec[0][big_index as usize] = 1;
                if big_index > 0 {
                    big_index -= 1;
                }
            } else {
                diff -= (6 - vec[1][small_index as usize]);
                vec[1][small_index as usize] = 6;
                if small_index < vec[1].len() as i32 - 1 {
                    small_index += 1;
                }
            }
            cnt += 1;
        }

        cnt
    }
}

struct Solution;

fn main() {}