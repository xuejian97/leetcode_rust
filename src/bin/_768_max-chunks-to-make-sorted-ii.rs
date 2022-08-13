/*
You are given an integer array arr.

We split arr into some number of chunks (i.e., partitions), and individually sort each chunk. After concatenating them, the result should equal the sorted array.

Return the largest number of chunks we can make to sort the array.



Example 1:

Input: arr = [5,4,3,2,1]
Output: 1
Explanation:
Splitting into two or more chunks will not return the required result.
For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.
Example 2:

Input: arr = [2,1,3,4,4]
Output: 4
Explanation:
We can split into two chunks, such as [2, 1], [3, 4, 4].
However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.


Constraints:

1 <= arr.length <= 2000
0 <= arr[i] <= 108

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/max-chunks-to-make-sorted-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for x in arr {
            if stack.is_empty() || &x >= stack.last().unwrap() {
                stack.push(x);
            } else {
                let max = stack.pop().unwrap();
                while !stack.is_empty() && stack.last().unwrap() > &x {
                    stack.pop();
                }
                stack.push(max);
            }
        }
        stack.len() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::max_chunks_to_sorted(vec![5, 4, 3, 2, 1]),
        1
    );

    assert_eq!(
        Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]),
        4
    );

    assert_eq!(
        Solution::max_chunks_to_sorted(vec![1, 0, 1, 3, 2]),
        3
    );

    assert_eq!(
        Solution::max_chunks_to_sorted(vec![1, 1, 0, 0, 1]),
        2
    );
}