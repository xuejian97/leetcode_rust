/*
Given an m x n binary matrix mat, return the number of special positions in mat.

A position (i, j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).

 

Example 1:


Input: mat = [[1,0,0],[0,0,1],[1,0,0]]
Output: 1
Explanation: (1, 2) is a special position because mat[1][2] == 1 and all other elements in row 1 and column 2 are 0.
Example 2:


Input: mat = [[1,0,0],[0,1,0],[0,0,1]]
Output: 3
Explanation: (0, 0), (1, 1) and (2, 2) are special positions.
 

Constraints:

m == mat.length
n == mat[i].length
1 <= m, n <= 100
mat[i][j] is either 0 or 1.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/special-positions-in-a-binary-matrix
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut cnt = vec![0; mat[0].len()];

        for x in mat {
            let mut active_index_vec = vec![];
            for (i, &v) in x.iter().enumerate() {
                if v == 1 { active_index_vec.push(i); }
            }
            let cnt_v = if active_index_vec.len() == 1 { 1 } else {2};
            for i in active_index_vec {
                cnt[i] += cnt_v;
            }
        }

        cnt.iter().filter(|&c| c == &1).count() as i32
    }
}

struct Solution;

fn main() {}