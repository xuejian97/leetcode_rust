/*
We had some 2-dimensional coordinates, like "(1, 3)" or "(2, 0.5)". Then, we removed all commas, decimal points, and spaces and ended up with the string s.

For example, "(1, 3)" becomes s = "(13)" and "(2, 0.5)" becomes s = "(205)".
Return a list of strings representing all possibilities for what our original coordinates could have been.

Our original representation never had extraneous zeroes, so we never started with numbers like "00", "0.0", "0.00", "1.0", "001", "00.01", or any other number that can be represented with fewer digits. Also, a decimal point within a number never occurs without at least one digit occurring before it, so we never started with numbers like ".1".

The final answer list can be returned in any order. All coordinates in the final answer have exactly one space between them (occurring after the comma.)



Example 1:

Input: s = "(123)"
Output: ["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]
Example 2:

Input: s = "(0123)"
Output: ["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]
Explanation: 0.0, 00, 0001 or 00.01 are not allowed.
Example 3:

Input: s = "(00011)"
Output: ["(0, 0.011)","(0.001, 1)"]


Constraints:

4 <= s.length <= 12
s[0] == '(' and s[s.length - 1] == ')'.
The rest of s are digits.
 */
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let mut ans = vec![];
        for i in 1..s.len() - 2 {
            let left = &s[1..=i];
            let right = &s[i + 1..s.len() - 1];
            if !Solution::valid(left) || !Solution::valid(right) { continue; }
            let left_vec = Solution::legal_vec(left);
            let right_vec = Solution::legal_vec(right);
            for l in &left_vec {
                for r in &right_vec {
                    ans.push(format!("({}, {})", l, r));
                }
            }
        }
        ans
    }

    pub fn valid(s: &str) -> bool {
        return s == "0" || !(s.starts_with("0") && s.ends_with("0"));
    }

    pub fn legal_vec(s: &str) -> Vec<String> {
        use std::ops::Index;

        if s.len() == 1 || s.ends_with("0") { return vec![s.to_owned()]; }
        if s.starts_with("0") {
            return vec![s.replacen("0", "0.", 1)];
        }
        let mut ans = vec![];
        let mut end_index = s.len();
        for i in (0..s.len()).rev() {
            if s.index(i..=i) != "0" { break; }
            end_index = i;
        }
        ans.push(s.to_owned());
        for i in 0..end_index - 1 {
            ans.push(format!("{}.{}", &s[0..=i], &s[i + 1..s.len()]))
        }
        ans
    }
}

struct Solution;

fn main() {
    Solution::ambiguous_coordinates("(0110)".to_owned());
}