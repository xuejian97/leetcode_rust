/*
Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.

A mapping of digits to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.


 

Example 1:

Input: digits = "23"
Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
Example 2:

Input: digits = ""
Output: []
Example 3:

Input: digits = "2"
Output: ["a","b","c"]
 

Constraints:

0 <= digits.length <= 4
digits[i] is a digit in the range ['2', '9'].

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/letter-combinations-of-a-phone-number
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        use std::collections::HashMap;
        use std::ops::Index;

        if digits.len() == 0 { return vec![]; }

        let mut map: HashMap<u8, Vec<&str>> = HashMap::new();
        map.insert('2' as u8, vec!["a", "b", "c"]);
        map.insert('3' as u8, vec!["d", "e", "f"]);
        map.insert('4' as u8, vec!["g", "h", "i"]);
        map.insert('5' as u8, vec!["j", "k", "l"]);
        map.insert('6' as u8, vec!["m", "n", "o"]);
        map.insert('7' as u8, vec!["p", "q", "r", "s"]);
        map.insert('8' as u8, vec!["t", "u", "v"]);
        map.insert('9' as u8, vec!["w", "x", "y", "z"]);
        println!("{:#?}", map);
        let mut ans = vec![];

        fn walk(res: String, digits: &str, dict: &HashMap<u8, Vec<&str>>, func: &mut impl FnMut(String)) {
            if digits.len() == 0 {
                func(res.clone());
                return;
            }
            let digit = digits.as_bytes()[0];
            for &x in dict.get(&digit).unwrap().iter() {
                walk(format!("{}{}", res, x), digits.index(1..), &dict, func)
            };
        }

        walk("".to_owned(), digits.as_str(), &map, &mut |r: String| {
            ans.push(r);
        });

        ans
    }
}

struct Solution;

fn main() {
    let vec = Solution::letter_combinations("32".to_owned());
    println!("{:?}", vec);
}