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
    pub fn minimum_rounds(mut tasks: Vec<i32>) -> i32 {
        tasks.sort_unstable();
        let mut level_counts = vec![];
        {
            let mut c_x = -1;
            let mut last_index = 0;
            for (idx, &x) in tasks.iter().enumerate() {
                if c_x != x {
                    c_x = x;
                    let count = idx as i32 - last_index;
                    level_counts.push(count);
                    last_index = idx as i32;
                } else {
                    continue;
                }
            }
            level_counts.push(tasks.len() as i32 - last_index);
            if level_counts.contains(&1) { return -1; }
            level_counts.remove(0);
        }

        level_counts.iter().map(|&count| {
            match count % 3 {
                0 => { count / 3 }
                1 => { count / 3 - 1 + 2 }
                2 => { count / 3 + 1 }
                _ => { -1 }
            }
        }).sum()
    }
}

struct Solution;

fn main() {
    let i = Solution::minimum_rounds(vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4]);
    // let i = Solution::minimum_rounds(vec![2, 3, 3]);
    // let i = Solution::minimum_rounds(vec![1, 2, 1]);

    println!("{}", i);
}