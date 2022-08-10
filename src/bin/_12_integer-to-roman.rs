/*
Roman numerals are represented by seven different symbols:I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example,2 is written as IIin Roman numeral, just two one's added together. 12 is written asXII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

I can be placed before V (5) and X (10) to make 4 and 9.
X can be placed before L (50) and C (100) to make 40 and 90.
C can be placed before D (500) and M (1000) to make 400 and 900.
Given an integer, convert it to a roman numeral.



Example 1:

Input: num = 3
Output: "III"
Explanation: 3 is represented as 3 ones.
Example 2:

Input: num = 58
Output: "LVIII"
Explanation: L = 50, V = 5, III = 3.
Example 3:

Input: num = 1994
Output: "MCMXCIV"
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.


Constraints:

1 <= num <= 3999

来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/integer-to-roman
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let roman_numeral_dict = vec![
            ("I", "V", "X"),
            ("X", "L", "C"),
            ("C", "D", "M"),
            ("M", "", ""),
        ];
        let mut result = String::from("");

        let mut num = num;
        let mut i = 0;
        while num > 0 {
            let n = num % 10;
            num /= 10;
            if n == 4 {
                result = format!("{}{}{}", roman_numeral_dict[i].0, roman_numeral_dict[i].1, result);
            } else if n == 9 {
                result = format!("{}{}{}", roman_numeral_dict[i].0, roman_numeral_dict[i].2, result)
            } else if n < 4 {
                result = format!("{}{}", vec![roman_numeral_dict[i].0; n as usize].join(""), result);
            } else {
                let one_nums = n - 5;
                result = format!("{}{}{}", roman_numeral_dict[i].1, vec![roman_numeral_dict[i].0; one_nums as usize].join(""), result);
            }
            i += 1;
        }

        result
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::int_to_roman(3), "III".to_string());
    assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
}