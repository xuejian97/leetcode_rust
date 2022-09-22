/*
You are given an array of distinct integers arr and an array of integer arrays pieces, where the integers in pieces are distinct. Your goal is to form arr by concatenating the arrays in pieces in any order. However, you are not allowed to reorder the integers in each array pieces[i].

Return true if it is possible to form the array arr from pieces. Otherwise, return false.

 

Example 1:

Input: arr = [15,88], pieces = [[88],[15]]
Output: true
Explanation: Concatenate [15] then [88]
Example 2:

Input: arr = [49,18,16], pieces = [[16,18,49]]
Output: false
Explanation: Even though the numbers match, we cannot reorder pieces[0].
Example 3:

Input: arr = [91,4,64,78], pieces = [[78],[4,64],[91]]
Output: true
Explanation: Concatenate [91] then [4,64] then [78]
 

Constraints:

1 <= pieces.length <= arr.length <= 100
sum(pieces[i].length) == arr.length
1 <= pieces[i].length <= arr.length
1 <= arr[i], pieces[i][j] <= 100
The integers in arr are distinct.
The integers in pieces are distinct (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/check-array-formation-through-concatenation
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        use std::collections::HashMap;
        let mut dict: HashMap<i32, usize> = HashMap::new();
        for (idx, &v) in arr.iter().enumerate() {
            dict.insert(v, idx);
        }
        for sub_arr in pieces {
            if sub_arr.len() == 1 {
                if dict.contains_key(&sub_arr[0]) { continue; } else { return false; }
            }
            let idx = dict.get(&sub_arr[0]);
            if idx.is_none() { return false; }
            let mut idx = *idx.unwrap();
            for i in 1..sub_arr.len() {
                let next_idx = dict.get(&sub_arr[i]);
                if next_idx.is_none() { return false; }
                let next_idx = *next_idx.unwrap();
                if next_idx != idx + 1 { return false; }
                idx = next_idx;
            }
        }
        true
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::can_form_array(
            vec![15, 88],
            vec![vec![88], vec![15]],
        ),
        true
    );
    assert_eq!(
        Solution::can_form_array(
            vec![49, 18, 16],
            vec![vec![16, 18, 49]],
        ),
        false
    );
    assert_eq!(
        Solution::can_form_array(
            vec![91, 4, 64, 78],
            vec![vec![78], vec![4, 64], vec![91]],
        ),
        true
    );
}