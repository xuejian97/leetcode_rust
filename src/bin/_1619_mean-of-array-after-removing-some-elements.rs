/*
Given an integer array arr, return the mean of the remaining integers after removing the smallest 5% and the largest 5% of the elements.

Answers within 10-5 of the actual answer will be considered accepted.

 

Example 1:

Input: arr = [1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3]
Output: 2.00000
Explanation: After erasing the minimum and the maximum values of this array, all elements are equal to 2, so the mean is 2.
Example 2:

Input: arr = [6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]
Output: 4.00000
Example 3:

Input: arr = [6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]
Output: 4.77778
 

Constraints:

20 <= arr.length <= 1000
arr.length is a multiple of 20.
0 <= arr[i] <= 105


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/mean-of-array-after-removing-some-elements
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn trim_mean(mut arr: Vec<i32>) -> f64 {
        arr.sort_unstable();
        let mut sum = 0;
        let start_idx = arr.len() / 20;
        let end_idx = arr.len() / 20 * 19;
        for i in start_idx..end_idx {
            sum += arr[i];
        }

        let length = arr.len() as f64 / 20_f64 * 18_f64;
        sum as f64 / length
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::trim_mean(vec![1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3]), 2.0_f64);
    assert_eq!(Solution::trim_mean(vec![6,2,7,5,1,2,0,3,10,2,5,0,5,5,0,8,7,6,8,0]), 4.0_f64);
    assert_eq!(Solution::trim_mean(vec![6,0,7,0,7,5,7,8,3,4,0,7,8,1,6,8,1,1,2,4,8,1,9,5,4,3,8,5,10,8,6,6,1,0,6,10,8,2,3,4]), 4.77778_f64);
}