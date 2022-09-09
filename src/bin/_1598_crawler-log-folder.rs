/*
The Leetcode file system keeps a log each time some user performs a change folder operation.

The operations are described below:

"../" : Move to the parent folder of the current folder. (If you are already in the main folder, remain in the same folder).
"./" : Remain in the same folder.
"x/" : Move to the child folder named x (This folder is guaranteed to always exist).
You are given a list of strings logs where logs[i] is the operation performed by the user at the ith step.

The file system starts in the main folder, then the operations in logs are performed.

Return the minimum number of operations needed to go back to the main folder after the change folder operations.

 

Example 1:



Input: logs = ["d1/","d2/","../","d21/","./"]
Output: 2
Explanation: Use this change folder operation "../" 2 times and go back to the main folder.
Example 2:



Input: logs = ["d1/","d2/","./","d3/","../","d31/"]
Output: 3
Example 3:

Input: logs = ["d1/","../","../","../"]
Output: 0
 

Constraints:

1 <= logs.length <= 103
2 <= logs[i].length <= 10
logs[i] contains lowercase English letters, digits, '.', and '/'.
logs[i] follows the format described in the statement.
Folder names consist of lowercase English letters and digits.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/crawler-log-folder
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        use std::collections::VecDeque;
        let mut stack = VecDeque::new();
        for str in logs {
            match str.as_str() {
                "../" => { stack.pop_back(); }
                "./" => (),
                _ => { stack.push_back(str) }
            }
        }
        stack.len() as i32
    }
}

struct Solution;

fn main() {
    assert_eq!(
        Solution::min_operations(vec!["d1/".to_owned(), "d2/".to_owned(), "../".to_owned(), "d21/".to_owned(), "./".to_owned()]),
        2
    );
}