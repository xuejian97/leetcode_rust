/*
You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.


Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.

In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.



Example 1:


Input: n = 3
Output: 5
Explanation: The five different ways are show above.
Example 2:

Input: n = 1
Output: 1


Constraints:

1 <= n <= 1000
 */

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        let n = n as usize;
        let _mod = 1_000_000_007;
        let mut dp = vec![vec![0; 4]; n + 1];
        dp[0][3] = 1;

        for i in 1..=n {
            dp[i][0] = dp[i - 1][3];
            dp[i][1] = (dp[i - 1][0] + dp[i - 1][2]) % _mod;
            dp[i][2] = (dp[i - 1][0] + dp[i - 1][1]) % _mod;
            dp[i][3] = (((dp[i - 1][0] + dp[i - 1][1]) % _mod + dp[i - 1][2]) % _mod + dp[i - 1][3]) % _mod;
        }
        dp[n][3]
    }
}

struct Solution;

fn main() {
    Solution::num_tilings(3);
}