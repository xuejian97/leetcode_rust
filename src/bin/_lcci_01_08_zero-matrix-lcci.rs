/*
Write an algorithm such that if an element in an MxN matrix is 0, its entire row and column are set to 0.

 

Example 1:

Input:
[
  [1,1,1],
  [1,0,1],
  [1,1,1]
]
Output:
[
  [1,0,1],
  [0,0,0],
  [1,0,1]
]
Example 2:

Input:
[
  [0,1,2,0],
  [3,4,5,2],
  [1,3,1,5]
]
Output:
[
  [0,0,0,0],
  [0,4,5,0],
  [0,3,1,0]
]


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zero-matrix-lcci
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let mut zero_col_idx: HashSet<usize> = HashSet::new();
        let mut zero_row_idx: HashSet<usize> = HashSet::new();
        for (row_idx, row) in matrix.iter().enumerate() {
            for (col_idx, col) in row.iter().enumerate() {
                if *col == 0 {
                    zero_row_idx.insert(row_idx);
                    zero_col_idx.insert(col_idx);
                }
            }
        }

        for (row_idx, row) in matrix.iter_mut().enumerate() {
            if zero_row_idx.contains(&row_idx) {
                for col in row {
                    *col = 0;
                }
            } else {
                for (col_idx, col) in row.iter_mut().enumerate() {
                    if zero_col_idx.contains(&col_idx) {
                        *col = 0;
                    }
                }
            }
        }
    }
}

struct Solution;

fn main() {
    let mut matrix = vec![
        vec![0, 0, 0, 5],
        vec![4, 3, 1, 4],
        vec![0, 1, 1, 4],
        vec![1, 2, 1, 3],
        vec![0, 0, 1, 1],
    ];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}