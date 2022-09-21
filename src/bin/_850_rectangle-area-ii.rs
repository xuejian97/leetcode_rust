/*
You are given a 2D array of axis-aligned rectangles. Each rectangle[i] = [xi1, yi1, xi2, yi2] denotes the ith rectangle where (xi1, yi1) are the coordinates of the bottom-left corner, and (xi2, yi2) are the coordinates of the top-right corner.

Calculate the total area covered by all rectangles in the plane. Any area covered by two or more rectangles should only be counted once.

Return the total area. Since the answer may be too large, return it modulo 109 + 7.

 

Example 1:


Input: rectangles = [[0,0,2,2],[1,0,2,3],[1,0,3,1]]
Output: 6
Explanation: A total area of 6 is covered by all three rectangles, as illustrated in the picture.
From (1,1) to (2,2), the green and red rectangles overlap.
From (1,0) to (2,3), all three rectangles overlap.
Example 2:

Input: rectangles = [[0,0,1000000000,1000000000]]
Output: 49
Explanation: The answer is 1018 modulo (109 + 7), which is 49.
 

Constraints:

1 <= rectangles.length <= 200
rectanges[i].length == 4
0 <= xi1, yi1, xi2, yi2 <= 109

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/rectangle-area-ii
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn rectangle_area(rectangles: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        const MOD: i64 = 1000000007;
        let n = rectangles.len();
        let mut set = HashSet::new();
        for rect in &rectangles {
            set.insert(rect[1]);
            set.insert(rect[3]);
        }
        let mut hbound = set.into_iter().collect::<Vec<i32>>();
        hbound.sort_unstable();
        let m = hbound.len();
        let mut seg = vec![0; m - 1];
        let mut sweep: Vec<Vec<i32>> = vec![];
        for i in 0..n {
            sweep.push(vec![rectangles[i][0], i as i32, 1]);
            sweep.push(vec![rectangles[i][2], i as i32, -1]);
        }

        sweep.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[2].cmp(&b[2])
            }
        });

        let mut ans: i64 = 0;
        let mut i = 0;
        while i < sweep.len() {
            let mut j = i;
            while j + 1 < sweep.len() && sweep[i][0] == sweep[j + 1][0] {
                j += 1;
            }
            if j + 1 == sweep.len() {
                break;
            }
            for k in i..=j {
                let arr = &sweep[k];
                let idx = arr[1];
                let diff = arr[2];
                let left = rectangles[idx as usize][1];
                let right = rectangles[idx as usize][3];
                for x in 0..m - 1 {
                    if left <= hbound[x] && hbound[x + 1] <= right {
                        seg[x] += diff;
                    }
                }
            }

            let mut cover = 0;
            for k in 0..m - 1 {
                if seg[k] > 0 {
                    cover += hbound[k + 1] - hbound[k];
                }
            }
            ans += cover as i64 * (sweep[j + 1][0] as i64 - sweep[j][0] as i64);
            i = j;
            i += 1;
        }
        (ans % MOD) as i32
    }
}

struct Solution;

fn main() {

    assert_eq!(
        Solution::rectangle_area(
            vec![
                vec![0, 0, 2, 2],
                vec![1, 0, 2, 3],
                vec![1, 0, 3, 1],
            ]
        ),
        6
    );
}