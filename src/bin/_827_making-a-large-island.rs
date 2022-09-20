/*
You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.

Return the size of the largest island in grid after applying this operation.

An island is a 4-directionally connected group of 1s.

 

Example 1:

Input: grid = [[1,0],[0,1]]
Output: 3
Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.
Example 2:

Input: grid = [[1,1],[1,0]]
Output: 4
Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.
Example 3:

Input: grid = [[1,1],[1,1]]
Output: 4
Explanation: Can't change any 0 to 1, only one island with area = 4.
 

Constraints:

n == grid.length
n == grid[i].length
1 <= n <= 500
grid[i][j] is either 0 or 1.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/making-a-large-island
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, VecDeque, HashSet};
        let mut island_size: HashMap<i32, i32> = HashMap::new();
        let mut island_tag = 2;
        let height = grid.len();
        let width = grid[0].len();
        let mut max_island_size = 0;

        for i in 0..height {
            for j in 0..width {
                if grid[i][j] == 1 {
                    let mut deque = VecDeque::new();
                    deque.push_back((i, j));
                    grid[i][j] = island_tag;

                    while deque.len() > 0 {
                        let (item_i, item_j) = deque.pop_front().unwrap();
                        match island_size.get_mut(&island_tag) {
                            Some(size) => { *size += 1 }
                            None => { island_size.insert(island_tag, 1); }
                        }
                        if item_i > 0 && grid[item_i - 1][item_j] == 1 {
                            deque.push_back((item_i - 1, item_j));
                            grid[item_i - 1][item_j] = island_tag;
                        }
                        if item_i < height - 1 && grid[item_i + 1][item_j] == 1 {
                            deque.push_back((item_i + 1, item_j));
                            grid[item_i + 1][item_j] = island_tag;
                        }
                        if item_j > 0 && grid[item_i][item_j - 1] == 1 {
                            deque.push_back((item_i, item_j - 1));
                            grid[item_i][item_j - 1] = island_tag;
                        }
                        if item_j < width - 1 && grid[item_i][item_j + 1] == 1 {
                            deque.push_back((item_i, item_j + 1));
                            grid[item_i][item_j + 1] = island_tag;
                        }
                    }
                    let current_island_size = *island_size.get(&island_tag).unwrap();
                    if current_island_size > max_island_size {
                        max_island_size = current_island_size
                    }
                    island_tag += 1;
                }
            }
        }

        for i in 0..height {
            for j in 0..grid[i].len() {
                if grid[i][j] == 0 {
                    let mut set = HashSet::new();
                    if i > 0 && grid[i - 1][j] != 0 {
                        set.insert(grid[i - 1][j]);
                    }
                    if i < height - 1 && grid[i + 1][j] != 0 {
                        set.insert(grid[i + 1][j]);
                    }
                    if j > 0 && grid[i][j - 1] != 0 {
                        set.insert(grid[i][j - 1]);
                    }
                    if j < width - 1 && grid[i][j + 1] != 0 {
                        set.insert(grid[i][j + 1]);
                    }
                    let current_island_size = set.iter().map(|tag| { return island_size.get(tag).unwrap(); }).sum::<i32>() + 1;
                    if current_island_size > max_island_size {
                        max_island_size = current_island_size;
                    }
                }
            }
        }

        max_island_size
    }
}

struct Solution;

fn main() {
    // assert_eq!(
    //     Solution::largest_island(
    //         vec![
    //             vec![1, 0],
    //             vec![0, 1],
    //         ]
    //     ),
    //     3
    // );
    assert_eq!(
        Solution::largest_island(
            vec![
                vec![1, 1],
                vec![1, 1],
            ]
        ),
        4
    );
}