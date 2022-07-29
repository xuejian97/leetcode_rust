/*
You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.



Example 1:

Input: coins = [1,2,5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1
Example 2:

Input: coins = [2], amount = 3
Output: -1
Example 3:

Input: coins = [1], amount = 0
Output: 0


Constraints:

1 <= coins.length <= 12
1 <= coins[i] <= 231 - 1
0 <= amount <= 104

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/coin-change
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let m = amount as usize;

        let mut f = vec![-1; m + 1];

        f[0] = 0;
        for i in 1..=m {
            let mut min = i32::MAX;
            for &coin in coins.iter() {
                if coin > i as i32 { break; }
                if f[i - coin as usize] != -1 {
                    min = i32::min(min, f[i - coin as usize]);
                }

            }

            f[i] = if min == i32::MAX { -1 } else { 1 + min };
        }

        f[m]
    }

}

struct Solution;
fn main() {
    let i = Solution::coin_change(vec![2, 5, 7], 27);
    println!("{}", i);
}