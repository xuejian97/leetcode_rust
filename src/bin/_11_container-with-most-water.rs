/*
You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).

Find two lines that together with the x-axis form a container, such that the container contains the most water.

Return the maximum amount of water a container can store.

Notice that you may not slant the container.



Example 1:


Input: height = [1,8,6,2,5,4,8,3,7]
Output: 49
Explanation: The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the container can contain is 49.
Example 2:

Input: height = [1,1]
Output: 1


Constraints:

n == height.length
2 <= n <= 105
0 <= height[i] <= 104

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/container-with-most-water
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {

        let mut left_idx = 0;
        let mut right_idx = height.len() - 1;

        let mut res = 0;

        while left_idx != right_idx {

            let l = (right_idx - left_idx) as i32;

            let area = if height[left_idx] < height[right_idx] {
                let i = left_idx;
                left_idx += 1;
                height[i]
            } else {
                let i = right_idx;
                right_idx -= 1;
                height[i]
            } * l;

            if area > res { res = area }

        }

        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    println!("{}", Solution::max_area(vec![1, 1]));
}