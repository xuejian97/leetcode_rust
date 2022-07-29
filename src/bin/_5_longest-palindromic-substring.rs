/*
Given a string s, return the longest palindromic substring in s.



Example 1:

Input: s = "babad"
Output: "bab"
Explanation: "aba" is also a valid answer.
Example 2:

Input: s = "cbbd"
Output: "bb"


Constraints:

1 <= s.length <= 1000
s consist of only digits and English letters.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/longest-palindromic-substring
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 { return s }
        let length = s.len();
        let mut f: Vec<(usize, usize)> = vec![];
        let str_bytes = s.clone().into_bytes();
        // even
        for i in 0..length - 1 {
            if str_bytes[i] == str_bytes[i + 1] {
                f.push((i, i + 1));
            };
        }

        // odd
        for i in 1..length - 1 {
            if str_bytes[i - 1] == str_bytes[i + 1] {
                f.push((i - 1, i + 1));
            };
        }

        if f.len() == 0 { return s[0..=0].to_owned(); }

        let mut x: Vec<(usize, usize)> = f.iter().map(move |(l, r)| {
            find_max_palindromic(&str_bytes, *l, *r)
        }).collect();

        x.sort_by(|&a, &b|{
            (b.1 - b.0).cmp(&(a.1 - a.0))
        });

        s[x[0].0..=x[0].1].to_owned()
    }
}

pub fn find_max_palindromic(str_bytes: &Vec<u8>, l: usize, r: usize) -> (usize, usize){
    if l == 0 || r == str_bytes.len() - 1 || str_bytes[l - 1] != str_bytes[r + 1] {
        return (l, r);
    }

    find_max_palindromic(str_bytes, l - 1, r + 1)

}

struct Solution;

fn main() {
    let string = Solution::longest_palindrome(String::from("babad"));
    println!("{}", string);
}