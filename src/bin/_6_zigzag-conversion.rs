/*
The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R
And then read line by line: "PAHNAPLSIIGYIR"

Write the code that will take a string and make this conversion given a number of rows:

string convert(string s, int numRows);
 

Example 1:

Input: s = "PAYPALISHIRING", numRows = 3
Output: "PAHNAPLSIIGYIR"
Example 2:

Input: s = "PAYPALISHIRING", numRows = 4
Output: "PINALSIGYAHRPI"
Explanation:
P     I    N
A   L S  I G
Y A   H R
P     I
Example 3:

Input: s = "A", numRows = 1
Output: "A"
 

Constraints:

1 <= s.length <= 1000
s consists of English letters (lower-case and upper-case), ',' and '.'.
1 <= numRows <= 1000


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/zigzag-conversion
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */


impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        use std::ops::Index;

        if num_rows == 1 {
            return s;
        }
        let mut res = String::from("");
        for i in 0..num_rows {
            let mut index = i;
            let mut flag = true;
            while index < s.len() as i32 {
                res.push_str(s.index(index as usize..index as usize + 1));
                if i == num_rows - 1 || i == 0 {
                    index += 2 * num_rows - 2;
                } else {
                    if flag {
                        index += 2 * (num_rows - i) - 2;
                    } else {
                        index += 2 * (i + 1) - 2;
                    }

                    flag = !flag;
                }
            }

        }

        res
    }
}

struct Solution;

fn main() {
    let res = Solution::convert("PAYPALISHIRING".to_owned(), 4);
    assert_eq!("PINALSIGYAHRPI".to_owned(), res);
}