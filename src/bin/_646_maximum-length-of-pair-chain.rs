/*
You are given an array of n pairs pairs where pairs[i] = [lefti, righti] and lefti < righti.

A pair p2 = [c, d] follows a pair p1 = [a, b] if b < c. A chain of pairs can be formed in this fashion.

Return the length longest chain which can be formed.

You do not need to use up all the given intervals. You can select pairs in any order.

 

Example 1:

Input: pairs = [[1,2],[2,3],[3,4]]
Output: 2
Explanation: The longest chain is [1,2] -> [3,4].
Example 2:

Input: pairs = [[1,2],[7,8],[4,5]]
Output: 3
Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].
 

Constraints:

n == pairs.length
1 <= n <= 1000
-1000 <= lefti < righti <= 1000

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/maximum-length-of-pair-chain
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        pairs.sort_by(|v1, v2| {
            v1[1].cmp(&v2[1])
        });

        println!("{:?}", pairs);

        let mut cur_val = i32::MIN;
        let mut cur_tail_min = i32::MIN;
        let mut res = 0;

        for v in pairs {
            if v[0] > cur_tail_min {
                res += 1;
                cur_val = v[0];
                cur_tail_min = v[1];
                continue;
            }
            if v[0] == cur_val && v[1] < cur_tail_min {
                cur_tail_min = v[1];
            }
        }

        res
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
    // [,,,[-1,4],[-6,-2],[-9,8],[-5,3],[0,3]]
    assert_eq!(
        Solution::find_longest_chain(vec![
            vec![-6, 9],
            vec![1, 6],
            vec![8, 10],
            vec![-1, 4],
            vec![-6, -2],
            vec![-9, 8],
            vec![-5, 3],
            vec![0, 3],
        ]),
        3
    );
}