/*
Given aninteger n, return a string with ncharacters such that each character in such string occurs an odd number of times.

The returned string must contain only lowercase English letters. If there are multiples valid strings, return any of them. 



Example 1:

Input: n = 4
Output: "pppz"
Explanation: "pppz" is a valid string since the character 'p' occurs three times and the character 'z' occurs once. Note that there are many other valid strings such as "ohhh" and "love".
Example 2:

Input: n = 2
Output: "xy"
Explanation: "xy" is a valid string since the characters 'x' and 'y' occur once. Note that there are many other valid strings such as "ag" and "ur".
Example 3:

Input: n = 7
Output: "holasss"


Constraints:

1 <= n <= 500

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/generate-a-string-with-characters-that-have-odd-counts
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        let mut res:String = String::from("");
        if n % 2 == 0 {
            res.push_str(&vec!["a"; n as usize - 1].join(""));
            res.push('b');
        } else {
            res.push_str(&vec!["a"; n as usize].join(""));
        }

        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::generate_the_string(4));
    println!("{}", Solution::generate_the_string(2));
    println!("{}", Solution::generate_the_string(7));
}