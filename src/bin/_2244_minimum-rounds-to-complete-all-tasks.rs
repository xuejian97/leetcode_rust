/*
You are given a 0-indexed integer array tasks, where tasks[i] represents the difficulty level of a task. In each round, you can complete either 2 or 3 tasks of the same difficulty level.

Return the minimum rounds required to complete all the tasks, or -1 if it is not possible to complete all the tasks.



Example 1:

Input: tasks = [2,2,3,3,2,4,4,4,4,4]
Output: 4
Explanation: To complete all the tasks, a possible plan is:
- In the first round, you complete 3 tasks of difficulty level 2. 
- In the second round, you complete 2 tasks of difficulty level 3. 
- In the third round, you complete 3 tasks of difficulty level 4. 
- In the fourth round, you complete 2 tasks of difficulty level 4.  
It can be shown that all the tasks cannot be completed in fewer than 4 rounds, so the answer is 4.
Example 2:

Input: tasks = [2,3,3]
Output: -1
Explanation: There is only 1 task of difficulty level 2, but in each round, you can only complete either 2 or 3 tasks of the same difficulty level. Hence, you cannot complete all the tasks, and the answer is -1.


Constraints:

1 <= tasks.length <= 105
1 <= tasks[i] <= 109

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/minimum-rounds-to-complete-all-tasks
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    // TODO slow
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let mut map = std::collections::HashMap::new();
        tasks.iter().for_each(|&i| {
            match map.get_mut(&i) {
                Some(&mut count) => { map.insert(i, count + 1); }
                None => { map.insert(i, 1); }
            };
        });

        let resolve_type: Vec<i32> = vec![2, 3];
        // f[x] = min{f[x-2] + 1, f[x-3] + 1}
        // f[0] = 0
        let count_list = map.iter().map(|(_, &count)| {
            let m = count as usize;
            let mut dp = vec![-1; m + 1];
            dp[0] = 0;
            for i in 1..=m {
                let mut min = i32::MAX;
                for &rc in &resolve_type {
                    if rc > i as i32 { break; }
                    if dp[i - rc as usize] != -1 {
                        min = i32::min(min, dp[i - rc as usize]);
                    }
                }

                dp[i] = if min == i32::MAX { -1 } else { min + 1 }
            }
            dp[m]
        }).collect::<Vec<_>>();

        if count_list.contains(&-1) {
            -1
        } else {
            count_list.iter().sum()
        }
    }
}

struct Solution;

fn main() {
    // let i = Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]);
    let i = Solution::minimum_rounds(vec![2, 3, 3]);
    println!("{}", i);
}