/*
Given a sorted integer array arr, two integers k and x, return the k closest integers to x in the array. The result should also be sorted in ascending order.

An integer a is closer to x than an integer b if:

|a - x| < |b - x|, or
|a - x| == |b - x| and a < b
 

Example 1:

Input: arr = [1,2,3,4,5], k = 4, x = 3
Output: [1,2,3,4]
Example 2:

Input: arr = [1,2,3,4,5], k = 4, x = -1
Output: [1,2,3,4]
 

Constraints:

1 <= k <= arr.length
1 <= arr.length <= 104
arr is sorted in ascending order.
-104 <= arr[i], x <= 104

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/find-k-closest-elements
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (mut left, mut right): (i32, i32) = (arr.len() as i32 - k, arr.len() as i32 - 1);
        for (idx, &v) in arr.iter().enumerate() {
            if v >= x {
                let mut round = k;
                if v == x {
                    left = idx as i32;
                    right = idx as i32;
                    round = k - 1;
                } else {
                    left = idx as i32;
                    right = idx as i32 - 1;
                }

                for _ in 0..round {
                    if left == 0 {
                        right += 1;
                        continue;
                    }
                    if right == arr.len() as i32 - 1 {
                        left -= 1;
                        continue;
                    }
                    if Solution::a_closer_x_than_b(
                        arr[left as usize - 1],
                        arr[right as usize + 1],
                        x,
                    ) {
                        left -= 1;
                    } else {
                        right += 1;
                    }
                }
                break;
            }
        }

        arr[left as usize..=right as usize].to_vec()
    }

    pub fn a_closer_x_than_b(a: i32, b: i32, x: i32) -> bool {
        (a - x).abs() < (b - x).abs() || ((a - x).abs() == (b - x).abs() && a < b)
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_closest_elements(
            vec![1, 2, 3, 4, 5],
            4,
            3,
        ),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(
            vec![1, 2, 3, 4, 5],
            4,
            -1,
        ),
        vec![1, 2, 3, 4]
    );
    assert_eq!(
        Solution::find_closest_elements(
            vec![1, 1, 1, 10, 10, 10],
            1,
            9,
        ),
        vec![10]
    );
}