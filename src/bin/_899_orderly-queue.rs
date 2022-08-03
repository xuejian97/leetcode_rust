/*
You are given a string s and an integer k. You can choose one of the first k letters of s and append it at the end of the string..

Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.



Example 1:

Input: s = "cba", k = 1
Output: "acb"
Explanation:
In the first move, we move the 1st character 'c' to the end, obtaining the string "bac".
In the second move, we move the 1st character 'b' to the end, obtaining the final result "acb".
Example 2:

Input: s = "baaca", k = 3
Output: "aaabc"
Explanation:
In the first move, we move the 1st character 'b' to the end, obtaining the string "aacab".
In the second move, we move the 3rd character 'c' to the end, obtaining the final result "aaabc".


Constraints:

1 <= k <= s.length <= 1000
s consist of lowercase English letters.

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/orderly-queue
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        match k {
            1 => {
                let mut x: String = "".to_owned();
                for i in 0..s.len() {
                    let mut str = s[i..s.len()].to_owned();
                    str.push_str(&s[0..i]);
                    if x == "" {
                        x = str;
                    } else {
                        match str.cmp(&x) {
                            std::cmp::Ordering::Less => {
                                x = str
                            }
                            _ => {}
                        }
                    }
                }
                x
            }
            _ => {
                let mut x: Vec<&str> = s
                    .split("")
                    .filter(|&a| a != "")
                    .collect();
                x.sort_by(|&a, &b| {
                    a.cmp(b)
                });
                x.join("")
            }
        }
    }
}

struct Solution;

fn main() {
    let s = String::from("cba");
    let k = 1;
    // let s = String::from("baaca");
    // let k = 3;

    let string = Solution::orderly_queue(s, k);
    println!("{}", string);


}