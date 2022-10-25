/*
You are given an n x n binary matrix grid where 1 represents land and 0 represents water.

An island is a 4-directionally connected group of 1's not connected to any other 1's. There are exactly two islands in grid.

You may change 0's to 1's to connect the two islands to form one island.

Return the smallest number of 0's you must flip to connect the two islands.



Example 1:

Input: grid = [[0,1],[1,0]]
Output: 1
Example 2:

Input: grid = [[0,1,0],[0,0,0],[0,0,1]]
Output: 2
Example 3:

Input: grid = [[1,1,1,1,1],[1,0,0,0,1],[1,0,1,0,1],[1,0,0,0,1],[1,1,1,1,1]]
Output: 1


Constraints:

n == grid.length == grid[i].length
2 <= n <= 100
grid[i][j] is either 0 or 1.
There are exactly two islands in grid.
 */
impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        let mut island_id = 0;
        let mut island_map = vec![
            vec![],
            vec![],
        ];

        for row_index in 0..grid.len() {
            for col_index in 0..grid.len() {
                let v = grid[row_index][col_index];
                if v == 1 {
                    let mut queue = VecDeque::new();
                    queue.push_back((row_index, col_index));

                    while !queue.is_empty() {
                        for _ in 0..queue.len() {
                            let (ri, ci) = queue.pop_front().unwrap();
                            if grid[ri][ci] != 1 { continue; }
                            grid[ri][ci] += 2_i32.pow(island_id);
                            island_map[island_id as usize].push((ri, ci));

                            if ri > 0 {
                                if grid[ri - 1][ci] == 1 {
                                    queue.push_back((ri - 1, ci));
                                }
                            }
                            if ri < grid.len() - 1 {
                                if grid[ri + 1][ci] == 1 {
                                    queue.push_back((ri + 1, ci));
                                }
                            }
                            if ci < grid.len() - 1 {
                                if grid[ri][ci + 1] == 1 {
                                    queue.push_back((ri, ci + 1));
                                }
                            }
                            if ci > 0 {
                                if grid[ri][ci - 1] == 1 {
                                    queue.push_back((ri, ci - 1));
                                }
                            }
                        }
                    }

                    island_id += 1;
                }
            }
        }


        let mut shortest_bridge = i32::MAX;
        for island_1 in &island_map[0] {
            for island_2 in &island_map[1] {
                let cur_bridge = (island_1.0 as i32 - island_2.0 as i32).abs()
                    + (island_1.1 as i32 - island_2.1 as i32).abs()
                    - 1;
                if cur_bridge < shortest_bridge {
                    shortest_bridge = cur_bridge;
                }
            }
        }

        shortest_bridge
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::shortest_bridge(
        vec![
            vec![0, 1],
            vec![1, 0],
        ]
    ), 1);
    assert_eq!(Solution::shortest_bridge(
        vec![
            vec![0, 1, 0],
            vec![0, 0, 0],
            vec![0, 0, 1],
        ]
    ), 2);
    assert_eq!(Solution::shortest_bridge(
        vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ]
    ), 1);
    assert_eq!(Solution::shortest_bridge(
        vec![
            vec![0, 0, 1, 0, 1],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ]
    ), 1);
    assert_eq!(Solution::shortest_bridge(
        vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 1, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0, 0, 0],
        ]
    ), 1);
}