/*
Given an integer n, return a binary string representing its representation in base -2.

Note that the returned string should not have leading zeros unless the string is "0".



Example 1:

Input: n = 2
Output: "110"
Explantion: (-2)2 + (-2)1 = 2
Example 2:

Input: n = 3
Output: "111"
Explantion: (-2)2 + (-2)1 + (-2)0 = 3
Example 3:

Input: n = 4
Output: "100"
Explantion: (-2)2 = 4


Constraints:

0 <= n <= 109
 */
impl Solution {
    pub fn base_neg2(mut n: i32) -> String {
        if n <= 1 {
            return n.to_string();
        }

        let mut bin_vec = vec![];
        while n > 0 {
            bin_vec.push(n % 2);
            n /= 2;
        }

        let mut i = 1usize;
        let mut k = bin_vec.len() - 1;

        while i <= k {
            if i % 2 == 0 {
                i += 1;
                continue;
            }
            if bin_vec[i] == 0 {
                i += 1;
                continue;
            }
            if i == k {
                bin_vec.push(1);
                break;
            }
            if bin_vec[i + 1] == 0 {
                bin_vec[i + 1] = 1;
                i += 1;
                continue;
            }

            let mut j = i;
            while j < k && bin_vec[j + 1] == 1 {
                bin_vec[j + 1] = 0;
                j += 1;
            }
            if j == k {
                bin_vec.push(1);
                k += 1;
            } else {
                bin_vec[j + 1] = 1;
            }
            i += 1;
        }


        bin_vec.reverse();
        let mut res = "".to_string();
        for i in bin_vec {
            res.push_str(i.to_string().as_str());
        }

        res
    }
}

struct Solution;

fn main() {
    println!("{}", Solution::base_neg2(6));
}