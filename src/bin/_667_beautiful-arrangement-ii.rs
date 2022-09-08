/*
Given two integers n and k, construct a list answer that contains n different positive integers ranging from 1 to n and obeys the following requirement:

Suppose this list is answer = [a1, a2, a3, ... , an], then the list [|a1 - a2|, |a2 - a3|, |a3 - a4|, ... , |an-1 - an|] has exactly k distinct integers.
Return the list answer. If there multiple valid answers, return any of them.

 

Example 1:

Input: n = 3, k = 1
Output: [1,2,3]
Explanation: The [1,2,3] has three different positive integers ranging from 1 to 3, and the [1,1] has exactly 1 distinct integer: 1
Example 2:

Input: n = 3, k = 2
Output: [1,3,2]
Explanation: The [1,3,2] has three different positive integers ranging from 1 to 3, and the [2,1] has exactly 2 distinct integers: 1 and 2.
 

Constraints:

1 <= k < n <= 104

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/beautiful-arrangement-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let left_len = n - k;
        for i in 1..=left_len {
            res.push(i);
        }

        let mut left = left_len + 1;
        let mut right = n;
        let mut i = 0;
        while left <= right {
            if i % 2 == 0 {
                res.push(right);
                right -= 1;
            } else {
                res.push(left);
                left += 1;
            }
            i += 1;
        }
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::construct_array(5, 4),
        vec![1, 5, 2, 4, 3]
    );
}