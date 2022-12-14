/*
Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

Symbol       Value
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

I can be placed before V (5) and X (10) to make 4 and 9. 
X can be placed before L (50) and C (100) to make 40 and 90. 
C can be placed before D (500) and M (1000) to make 400 and 900.
Given a roman numeral, convert it to an integer.

 

Example 1:

Input: s = "III"
Output: 3
Explanation: III = 3.
Example 2:

Input: s = "LVIII"
Output: 58
Explanation: L = 50, V= 5, III = 3.
Example 3:

Input: s = "MCMXCIV"
Output: 1994
Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
 

Constraints:

1 <= s.length <= 15
s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
It is guaranteed that s is a valid roman numeral in the range [1, 3999].


来源：力扣（LeetCode）
链接：https://leetcode.cn/problems/roman-to-integer
著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
 */

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert('I', 1);
        map.insert('V', 5);
        map.insert('X', 10);
        map.insert('L', 50);
        map.insert('C', 100);
        map.insert('D', 500);
        map.insert('M', 1000);

        let mut res = 0;
        let bytes = s.as_bytes();
        for (i, &x) in bytes.iter().enumerate() {
            let key = x as char;
            res += map.get(&key).unwrap();

            if i == 0 { continue; }
            if (key == 'V' || key == 'X') && bytes[i - 1] as char == 'I' {
                res -= 2;
            }
            if (key == 'L' || key == 'C') && bytes[i - 1] as char == 'X' {
                res -= 20;
            }
            if (key == 'D' || key == 'M') && bytes[i - 1] as char == 'C' {
                res -= 200;
            }
        }

        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_owned()), 3);
    assert_eq!(Solution::roman_to_int("LVIII".to_owned()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_owned()), 1994);
}