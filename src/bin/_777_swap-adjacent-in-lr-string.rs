/*
In a string composed of 'L', 'R', and 'X' characters, like "RXXLRXRXL", a move consists of either replacing one occurrence of "XL" with "LX", or replacing one occurrence of "RX" with "XR". Given the starting string start and the ending string end, return True if and only if there exists a sequence of moves to transform one string to the other.

 

Example 1:

Input: start = "RXXLRXRXL", end = "XRLXXRRLX"
Output: true
Explanation: We can transform start to end following these steps:
RXXLRXRXL ->
XRXLRXRXL ->
XRLXRXRXL ->
XRLXXRRXL ->
XRLXXRRLX
Example 2:

Input: start = "X", end = "L"
Output: false
 

Constraints:

1 <= start.length <= 104
start.length == end.length
Both start and end will only consist of characters in 'L', 'R', and 'X'.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/swap-adjacent-in-lr-string
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        use std::ops::Index;

        let n = start.len();
        let (mut i, mut j) = (0, 0);
        while i < n && j < n {
            while i < n && start.index(i..=i) == "X" {
                i += 1;
            }
            while j < n && end.index(j..=j) == "X" {
                j += 1;
            }
            if i < n && j < n {
                if start.index(i..=i) != end.index(j..=j) {
                    return false;
                }
                let c = start.index(i..=i);
                if (c == "L" && i < j) || (c == "R" && i > j) {
                    return false;
                }
                i += 1;
                j += 1;
            }
        }
        while i < n {
            if start.index(i..=i) != "X" {
                return false;
            }
            i += 1;
        }
        while j < n {
            if end.index(j..=j) != "X" {
                return false;
            }
            j += 1;
        }
        true
    }
}

struct Solution;

fn main() {
    let transform = Solution::can_transform(
        "LXXLXRLXXL".to_owned(),
        "XLLXRXLXLX".to_owned(),
    );

    println!("{}", transform);
}